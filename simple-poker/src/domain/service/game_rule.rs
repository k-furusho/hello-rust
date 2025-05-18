use crate::domain::model::bet::BetAction;
use crate::domain::model::game::{Game, GamePhase, GameVariant};
use crate::domain::model::player::Player;
use crate::domain::service::hand_evaluation::HandEvaluationService;

pub struct GameRuleService;

impl GameRuleService {
    // プレイヤーがとれるアクションを判定
    pub fn available_actions(game: &Game, player_index: usize) -> Vec<BetAction> {
        let mut actions = Vec::new();
        
        if game.current_phase() != GamePhase::Betting {
            return actions;
        }
        
        if player_index >= game.players().len() {
            return actions;
        }
        
        let player = &game.players()[player_index];
        
        // フォールド済みかオールインしているプレイヤーは何もできない
        if player.is_folded() || player.is_all_in() {
            return actions;
        }
        
        // フォールドは常に可能
        actions.push(BetAction::Fold);
        
        let current_bet = game.current_bet();
        let player_bet = player.current_bet();
        
        // 現在のベットに対する追加ベット額を計算
        let call_amount = current_bet.saturating_sub(player_bet);
        
        // チップがない場合はフォールドのみ
        if player.chips() == 0 {
            return actions;
        }
        
        // 現在のベットがゼロか、プレイヤーが既に最大ベット額を出している場合はチェック可能
        if current_bet == 0 || current_bet == player_bet {
            actions.push(BetAction::Check);
        }
        
        // プレイヤーが現在のベットをコールできるなら
        if call_amount > 0 && player.chips() >= call_amount {
            actions.push(BetAction::Call);
        }
        
        // レイズが可能（現在のベット額+最小ベット額以上のチップがある場合）
        let min_raise = game.big_blind();
        if player.chips() >= call_amount + min_raise {
            actions.push(BetAction::Raise);
        }
        
        // オールインは常に可能（ただしチップがある場合のみ）
        if player.chips() > 0 {
            actions.push(BetAction::AllIn);
        }
        
        actions
    }
    
    // プレイヤーのアクションを処理
    pub fn process_action(
        game: &mut Game,
        player_index: usize,
        action: BetAction,
        bet_amount: Option<u32>,
    ) -> Result<(), &'static str> {
        if game.current_phase() != GamePhase::Betting {
            return Err("ベッティングフェーズではありません");
        }
        
        if player_index >= game.players().len() {
            return Err("無効なプレイヤーインデックスです");
        }
        
        if player_index != game.current_player_index() {
            return Err("現在のプレイヤーのみアクションを実行できます");
        }
        
        let available_actions = Self::available_actions(game, player_index);
        if !available_actions.contains(&action) {
            return Err("そのアクションは現在実行できません");
        }
        
        match action {
            BetAction::Fold => {
                game.players_mut()[player_index].fold();
            },
            BetAction::Check => {
                // チェックは追加のベットなし
                // 現在のベットとプレイヤーのベットが同じであることを確認
                let current_bet = game.current_bet();
                let player_bet = game.players()[player_index].current_bet();
                if current_bet != player_bet {
                    return Err("チェックはできません。現在のベットをコールするかフォールドしてください");
                }
            },
            BetAction::Call => {
                let current_bet = game.current_bet();
                let player_bet = game.players()[player_index].current_bet();
                let call_amount = current_bet.saturating_sub(player_bet);
                
                if call_amount > 0 {
                    // コール額をベット
                    let amount_bet = game.players_mut()[player_index].place_bet(call_amount)?;
                    game.pot_mut().add(amount_bet);
                }
            },
            BetAction::Raise => {
                if let Some(raise_to) = bet_amount {
                    let current_bet = game.current_bet();
                    let player_bet = game.players()[player_index].current_bet();
                    let min_raise = current_bet + game.big_blind();
                    
                    // レイズは最低でも現在のベット+最小ベット額以上でなければならない
                    if raise_to < min_raise {
                        return Err("レイズは現在のベット額+最小ベット額以上でなければなりません");
                    }
                    
                    // プレイヤーがベットする額を計算（既にベットしている額を差し引く）
                    let additional_bet = raise_to.saturating_sub(player_bet);
                    if !game.players()[player_index].can_afford(additional_bet) {
                        return Err("そのレイズに必要なチップが足りません");
                    }
                    
                    // レイズを実行
                    let amount_bet = game.players_mut()[player_index].place_bet(additional_bet)?;
                    game.pot_mut().add(amount_bet);
                    
                    // 現在のベット額を更新
                    game.set_current_bet(raise_to);
                } else {
                    return Err("レイズにはベット額を指定する必要があります");
                }
            },
            BetAction::AllIn => {
                let player_chips = game.players()[player_index].chips();
                let player_bet = game.players()[player_index].current_bet();
                
                // オールイン
                let amount_bet = game.players_mut()[player_index].place_bet(player_chips)?;
                game.pot_mut().add(amount_bet);
                
                // プレイヤーの総ベット額（既存のベット+新規ベット）を計算
                let total_player_bet = player_bet + amount_bet;
                
                // 現在のベット額より多ければ、新しいベット額を設定
                if total_player_bet > game.current_bet() {
                    game.set_current_bet(total_player_bet);
                }
            },
        }
        
        // ラウンドが終了したかチェック
        Self::check_round_completion(game)?;
        
        Ok(())
    }
    
    // ラウンドが終了したかチェック
    fn check_round_completion(game: &mut Game) -> Result<(), &'static str> {
        if game.current_phase() != GamePhase::Betting {
            return Ok(());
        }
        
        // 次のプレイヤーに移動
        let next_player_index = Self::find_next_active_player(game);
        let current_player_index = game.current_player_index();
        
        // 参照の競合を解消するために、ここで別々のスコープで処理
        {
            let players = game.players_mut();
            if current_player_index < players.len() {
                players[current_player_index].reset_bet();
            }
        }
        
        // 次のプレイヤーが見つからない場合、またはラウンドを一周した場合
        let dealer_index = game.dealer_index();
        if next_player_index == dealer_index || next_player_index == current_player_index || Self::is_betting_round_complete(game) {
            game.end_betting_round()?;
        } else {
            let player_count = game.players().len();
            let next_index = (current_player_index + 1) % player_count;
            
            // 参照の競合を解消するために、ここで別々のスコープで処理
            {
                let players = game.players_mut();
                if next_index < players.len() {
                    players[next_index].reset_bet();
                }
            }
        }
        
        Ok(())
    }
    
    // 次のアクティブなプレイヤーを探す
    fn find_next_active_player(game: &Game) -> usize {
        let player_count = game.players().len();
        let mut index = (game.current_player_index() + 1) % player_count;
        let start_index = game.current_player_index();
        
        while index != start_index {
            let player = &game.players()[index];
            if !player.is_folded() && !player.is_all_in() {
                return index;
            }
            index = (index + 1) % player_count;
        }
        
        // 全員がフォールドかオールインの場合、または一周した場合
        start_index
    }
    
    // ベッティングラウンドが完了したかどうかを判定
    fn is_betting_round_complete(game: &Game) -> bool {
        // アクティブなプレイヤーの数を取得
        let active_players = game.players().iter()
            .filter(|p| !p.is_folded() && !p.is_all_in())
            .count();
        
        // アクティブなプレイヤーが0または1の場合、ラウンド終了
        if active_players <= 1 {
            return true;
        }
        
        // 全員のベット額が現在のベット額と一致しているかチェック
        let current_bet = game.current_bet();
        let all_matched = game.players().iter()
            .filter(|p| !p.is_folded() && !p.is_all_in())
            .all(|p| p.current_bet() == current_bet);
        
        all_matched
    }
    
    // ゲームの勝者を決定
    pub fn determine_winners(game: &Game) -> Vec<(usize, String)> {
        if game.current_phase() != GamePhase::Showdown {
            return Vec::new();
        }
        
        // フォールドしていないプレイヤーだけを対象にする
        let active_players: Vec<(usize, &Player)> = game.players().iter()
            .enumerate()
            .filter(|(_, p)| !p.is_folded())
            .collect();
        
        if active_players.len() == 1 {
            // 1人だけ残っている場合は自動的に勝者
            return vec![(active_players[0].0, active_players[0].1.name().to_string())];
        }
        
        // 各プレイヤーの手の強さを評価
        let mut player_strengths = Vec::new();
        
        for (idx, player) in active_players {
            let hand_strength = match game.variant() {
                GameVariant::FiveCardDraw => {
                    HandEvaluationService::evaluate_hand(player.hand().cards())
                },
                _ => {
                    HandEvaluationService::find_best_hand(
                        player.hand().cards(),
                        game.community_cards(),
                        game.variant()
                    )
                },
            };
            
            player_strengths.push((idx, player.name().to_string(), hand_strength));
        }
        
        // 最強の手を持つプレイヤーを見つける
        player_strengths.sort_by(|(_, _, a), (_, _, b)| b.cmp(a));
        
        let best_strength = &player_strengths[0].2;
        let winners: Vec<(usize, String)> = player_strengths.iter()
            .filter(|(_, _, strength)| strength == best_strength)
            .map(|&(idx, ref name, _)| (idx, name.clone()))
            .collect();
        
        winners
    }
    
    // ポットを分配
    pub fn distribute_pot(game: &mut Game) -> Result<Vec<(usize, u32)>, &'static str> {
        if game.current_phase() != GamePhase::Showdown {
            return Err("ショーダウンフェーズでのみポットを分配できます");
        }
        
        let winners = Self::determine_winners(game);
        if winners.is_empty() {
            return Err("勝者が決定できません");
        }
        
        let pot_amount = game.pot().total();
        let winner_count = winners.len();
        
        // 単純に等分（サイドポットは省略）
        let amount_per_winner = pot_amount / winner_count as u32;
        let mut distribution = Vec::new();
        
        for (idx, _) in &winners {
            game.players_mut()[*idx].add_chips(amount_per_winner);
            distribution.push((*idx, amount_per_winner));
        }
        
        // ポットをクリア
        game.pot_mut().clear();
        
        // ゲームフェーズを完了に設定
        // この部分は実際には別のサービスメソッドで行うべきかもしれません
        
        Ok(distribution)
    }
}
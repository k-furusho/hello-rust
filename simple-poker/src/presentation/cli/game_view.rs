use crate::domain::model::bet::BetAction;
use crate::domain::model::game::{Game, GamePhase, GameVariant};
use crate::domain::model::player::Player;
use crate::domain::model::error::DomainError;
use crate::domain::service::game_rule::GameRuleService;
use crate::domain::service::hand_evaluation::HandEvaluationService;
use crate::presentation::cli::input_handler::InputHandler;

pub struct GameView;

impl GameView {
    pub fn display_game_info(game: &Game) {
        println!("\n====================");
        println!("ポーカーゲーム: {}", game.variant().name());
        println!("フェーズ: {}", Self::phase_to_string(game.current_phase()));
        if let Some(round) = game.current_round() {
            println!("ラウンド: {}", round.name());
        }
        println!("ポット: {}チップ", game.pot().total());
        println!("現在のベット: {}チップ", game.current_bet());
        println!("====================\n");
    }
    
    pub fn display_players(game: &Game) {
        println!("\n--プレイヤー情報--");
        for (i, player) in game.players().iter().enumerate() {
            let status = if player.is_folded() {
                "（フォールド）"
            } else if player.is_all_in() {
                "（オールイン）"
            } else if i == game.current_player_index() {
                "（現在のプレイヤー）"
            } else {
                ""
            };
            
            println!(
                "{}. {} - {}チップ - 現在のベット: {}チップ {}{}",
                i + 1,
                player.name(),
                player.chips(),
                player.current_bet(),
                if player.is_dealer() { "【ディーラー】" } else { "" },
                status
            );
        }
        println!();
    }
    
    pub fn display_community_cards(game: &Game) {
        if game.community_cards().is_empty() {
            return;
        }
        
        println!("\n--コミュニティカード--");
        for card in game.community_cards() {
            print!("{} ", card);
        }
        println!("\n");
    }
    
    pub fn display_player_hand(player: &Player) {
        println!("\n--{}の手札--", player.name());
        for (i, card) in player.hand().cards().iter().enumerate() {
            println!("{}. {}", i + 1, card);
        }
        
        // プレイヤーの手の評価を表示（オプション）
        if !player.hand().is_empty() {
            let hand_strength = HandEvaluationService::evaluate_hand(player.hand().cards());
            println!("役: {}", hand_strength.rank());
        }
        println!();
    }
    
    pub fn get_player_action(game: &Game, player_index: usize) -> Result<(BetAction, Option<u32>), String> {
        let available_actions = GameRuleService::available_actions(game, player_index);
        if available_actions.is_empty() {
            return Err("有効なアクションがありません".to_string());
        }
        
        println!("\n--可能なアクション--");
        for (i, action) in available_actions.iter().enumerate() {
            println!("{}. {}", i + 1, Self::action_to_string(action));
        }
        
        let action_index = InputHandler::get_usize("アクションを選択")? - 1;
        if action_index >= available_actions.len() {
            return Err("無効な選択です".to_string());
        }
        
        let action = available_actions[action_index];
        let bet_amount = match action {
            BetAction::Raise => {
                let current_bet = game.current_bet();
                let min_raise = current_bet + game.big_blind();
                println!("最小レイズ額: {}チップ", min_raise);
                Some(InputHandler::get_u32("レイズ額")?)
            },
            _ => None,
        };
        
        Ok((action, bet_amount))
    }
    
    pub fn display_winners(game: &Game, winners: &[(usize, String)]) {
        println!("\n--ゲーム結果--");
        if winners.is_empty() {
            println!("勝者はいません");
            return;
        }
        
        let pot_per_winner = game.pot().total() / winners.len() as u32;
        
        for (idx, name) in winners {
            println!("勝者: {} - {}チップ獲得", name, pot_per_winner);
            
            // 勝利した手の表示
            if let Some(player) = game.players().get(*idx) {
                Self::display_player_hand(player);
            }
        }
        println!();
    }
    
    pub fn display_error<T: AsRef<str>>(error: T) {
        println!("\n[エラー] {}\n", error.as_ref());
    }
    
    pub fn phase_to_string(phase: GamePhase) -> &'static str {
        match phase {
            GamePhase::NotStarted => "準備中",
            GamePhase::Dealing => "配り中",
            GamePhase::Betting => "ベッティング",
            GamePhase::Drawing => "カード交換",
            GamePhase::Showdown => "ショーダウン",
            GamePhase::Complete => "終了",
        }
    }
    
    fn action_to_string(action: &BetAction) -> &'static str {
        match action {
            BetAction::Fold => "フォールド（降りる）",
            BetAction::Check => "チェック（様子見）",
            BetAction::Call => "コール（同額を賭ける）",
            BetAction::Raise => "レイズ（金額を上げる）",
            BetAction::AllIn => "オールイン（全てのチップを賭ける）",
        }
    }
    
    pub fn get_card_exchange(player: &Player) -> Result<Vec<usize>, String> {
        Self::display_player_hand(player);
        println!("交換したいカードの番号を入力してください（スペース区切り、何も入力せずにEnterでスキップ）");
        
        let input = InputHandler::get_string("");
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }
        
        let indices: Result<Vec<usize>, _> = input
            .split_whitespace()
            .map(|s| s.parse::<usize>().map(|n| n - 1)) // 0ベースに変換
            .collect();
            
        indices.map_err(|_| "無効な番号が含まれています".to_string())
    }
    
    pub fn prompt_for_game_variant() -> GameVariant {
        println!("\n--ポーカーの種類を選択--");
        println!("1. ファイブカードドロー");
        println!("2. テキサスホールデム");
        println!("3. オマハ");
        
        let choice = match InputHandler::get_menu_choice(3) {
            Ok(choice) => choice,
            Err(_) => {
                println!("無効な選択です。デフォルトでファイブカードドローを選択します。");
                1
            }
        };
        
        match choice {
            2 => GameVariant::TexasHoldem,
            3 => GameVariant::Omaha,
            _ => GameVariant::FiveCardDraw,
        }
    }
    
    pub fn prompt_for_blinds() -> (u32, u32) {
        let small_blind = match InputHandler::get_u32("スモールブラインド額") {
            Ok(amount) => amount,
            Err(_) => {
                println!("無効な額です。デフォルトで5に設定します。");
                5
            }
        };
        
        let big_blind = match InputHandler::get_u32("ビッグブラインド額") {
            Ok(amount) if amount > small_blind => amount,
            _ => {
                println!("無効な額です。スモールブラインドの2倍に設定します。");
                small_blind * 2
            }
        };
        
        (small_blind, big_blind)
    }
} 
use crate::application::usecase::save_game_usecase::{SaveGameParams, SaveGameUseCase};
use crate::application::usecase::load_game_usecase::LoadGameUseCase;
use crate::domain::repository::game_history_repository::GameHistoryRepository;
use crate::domain::error::GameError;

impl<G: GameRepository + Clone, P: PlayerRepository + Clone> MenuController<G, P> {
    fn display_main_menu(&self) {
        println!("\n--メインメニュー--");
        println!("1. 新しいゲームを作成");
        println!("2. ゲームをプレイ");
        println!("3. ゲーム一覧を表示");
        println!("4. 過去のゲーム履歴を表示");
        println!("5. 終了");
    }
    
    pub fn run(&mut self) {
        self.display_welcome();
        
        loop {
            self.display_main_menu();
            let choice = match InputHandler::get_menu_choice(5) {
                Ok(choice) => choice,
                Err(e) => {
                    GameView::display_error(&e);
                    continue;
                }
            };
            
            match choice {
                1 => self.create_new_game(),
                2 => self.play_game(),
                3 => self.view_games(),
                4 => self.view_game_history(),
                5 => break,
                _ => GameView::display_error("無効な選択です"),
            }
        }
        
        println!("ポーカーゲームを終了します。お疲れ様でした！");
    }
    
    fn handle_showdown_phase(&mut self, game_id: &GameId) {
        let game = match self.game_repository.find_by_id(game_id) {
            Some(game) => game,
            None => return,
        };
        
        println!("\n--ショーダウン--");
        
        // 全プレイヤーの手札を表示
        for player in game.players() {
            if !player.is_folded() {
                GameView::display_player_hand(player);
            }
        }
        
        // 勝者の決定と表示
        let winners = GameRuleService::determine_winners(&game);
        GameView::display_winners(&game, &winners);
        
        // ポットの分配
        let mut updated_game = game.clone();
        if let Err(e) = GameRuleService::distribute_pot(&mut updated_game) {
            GameView::display_error(&e);
        } else if let Err(e) = self.game_repository.save(&updated_game) {
            GameView::display_error(&e.to_string());
        }
        
        // ゲームを保存する
        println!("\nゲームを保存しますか？ (y/n)");
        if InputHandler::get_bool("") {
            self.save_game_result(game_id, &winners);
        }
        
        InputHandler::wait_for_enter();
    }
    
    fn save_game_result(&mut self, game_id: &GameId, winners: &[(usize, String)]) {
        // TODO: 実際の実装では、GameHistoryRepositoryの実装が必要
        println!("ゲームの保存は現在実装中です。");
        // 以下は実装例
        /*
        let winner_ids = winners.iter()
            .filter_map(|(idx, _)| game.players().get(*idx).map(|p| p.id().clone()))
            .collect();
        
        let params = SaveGameParams {
            game_id: game_id.clone(),
            winner_ids,
        };
        
        let history_repo = InMemoryGameHistoryRepository::new(); // 実際にはDIする
        let mut usecase = SaveGameUseCase::new(self.game_repository.clone(), history_repo);
        
        match usecase.execute(params) {
            Ok(_) => println!("ゲームが正常に保存されました"),
            Err(e) => GameView::display_error(&format!("ゲームの保存に失敗しました: {}", e)),
        }
        */
    }
    
    fn view_game_history(&self) {
        // TODO: 実際の実装では、GameHistoryRepositoryからデータを取得
        println!("\n--ゲーム履歴--");
        println!("この機能は現在実装中です。");
    }
} 
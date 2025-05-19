use crate::application::usecase::add_player_usecase::{AddPlayerParams, AddPlayerUseCase};
use crate::application::usecase::create_game_usecase::{CreateGameParams, CreateGameUseCase};
use crate::application::usecase::exchange_cards_usecase::{ExchangeCardsParams, ExchangeCardsUseCase};
use crate::application::usecase::place_bet_usecase::{PlaceBetParams, PlaceBetUseCase};
use crate::application::usecase::start_game_usecase::StartGameUseCase;
use crate::application::usecase::start_game_usecase::StartGameParams;
use crate::domain::model::game::{GameId, GamePhase};
use crate::domain::repository::game_repository::GameRepository;
use crate::domain::repository::player_repository::PlayerRepository;
use crate::domain::service::game_rule::GameRuleService;
use crate::presentation::cli::game_view::GameView;
use crate::presentation::cli::input_handler::InputHandler;
use crate::domain::model::event::{EventPublisher, EventSubscriber};

pub struct MenuController<G, P, E>
where 
    G: GameRepository + Clone,
    P: PlayerRepository + Clone,
    E: EventPublisher + EventSubscriber + Clone,
{
    game_repository: G,
    player_repository: P,
    event_publisher: E,
    current_game_id: Option<GameId>,
}

impl<G, P, E> MenuController<G, P, E>
where 
    G: GameRepository + Clone,
    P: PlayerRepository + Clone,
    E: EventPublisher + EventSubscriber + Clone,
{
    pub fn new(game_repository: G, player_repository: P, event_publisher: E) -> Self {
        Self {
            game_repository,
            player_repository,
            event_publisher,
            current_game_id: None,
        }
    }
    
    pub fn run(&mut self) {
        self.display_welcome();
        
        loop {
            self.display_main_menu();
            let choice = match InputHandler::get_menu_choice(4) {
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
                4 => break,
                _ => GameView::display_error("無効な選択です"),
            }
        }
        
        println!("ポーカーゲームを終了します。お疲れ様でした！");
    }
    
    fn display_welcome(&self) {
        println!("\n====================================");
        println!("      ポーカーゲームへようこそ！     ");
        println!("====================================\n");
    }
    
    fn display_main_menu(&self) {
        println!("\n--メインメニュー--");
        println!("1. 新しいゲームを作成");
        println!("2. ゲームをプレイ");
        println!("3. ゲーム一覧を表示");
        println!("4. 終了");
    }
    
    fn create_new_game(&mut self) {
        println!("\n--新しいゲームの作成--");
        
        // ゲームバリアントの選択
        let variant = GameView::prompt_for_game_variant();
        
        // ブラインド額の設定
        let (small_blind, big_blind) = GameView::prompt_for_blinds();
        
        let params = CreateGameParams {
            variant,
            small_blind,
            big_blind,
        };
        
        let mut usecase = CreateGameUseCase::new(
            self.game_repository.clone(),
            self.event_publisher.clone()
        );
        match usecase.execute(params) {
            Ok(game_id) => {
                println!("\nゲーム作成成功！ ゲームID: {}", game_id.value());
                self.current_game_id = Some(game_id);
                
                // プレイヤー追加画面に移行
                self.add_players();
            },
            Err(e) => GameView::display_error(&e),
        }
    }
    
    fn add_players(&mut self) {
        if self.current_game_id.is_none() {
            GameView::display_error("ゲームが選択されていません");
            return;
        }
        
        let game_id = self.current_game_id.clone().unwrap();
        
        println!("\n--プレイヤーの追加--");
        println!("最低2人のプレイヤーが必要です");
        
        loop {
            let player_name = InputHandler::get_string("プレイヤー名（終了するには'q'を入力）");
            if player_name.to_lowercase() == "q" {
                break;
            }
            
            let initial_chips = match InputHandler::get_u32("初期チップ数") {
                Ok(chips) => chips,
                Err(e) => {
                    GameView::display_error(&e);
                    continue;
                }
            };
            
            let params = AddPlayerParams {
                game_id: game_id.clone(),
                player_name,
                initial_chips,
            };
            
            let mut usecase = AddPlayerUseCase::new(
                self.game_repository.clone(),
                self.player_repository.clone()
            );
            
            match usecase.execute(params) {
                Ok(player_id) => println!("プレイヤー追加成功！ プレイヤーID: {}", player_id),
                Err(e) => GameView::display_error(&e),
            }
            
            // 現在のプレイヤー一覧を表示
            if let Some(game) = self.game_repository.find_by_id(&game_id) {
                println!("\n--現在のプレイヤー--");
                for (i, player) in game.players().iter().enumerate() {
                    println!("{}. {} ({}チップ)", i + 1, player.name(), player.chips());
                }
                
                if game.players().len() >= 2 {
                    println!("\nゲームを開始する準備ができました。");
                    println!("更にプレイヤーを追加するか、'q'を入力して次に進んでください。");
                } else {
                    println!("\nあと{}人のプレイヤーが必要です。", 2 - game.players().len());
                }
            }
        }
    }
    
    fn play_game(&mut self) {
        // ゲームの選択または現在のゲームの使用
        let game_id = match self.select_game() {
            Some(id) => id,
            None => return,
        };
        
        // ゲームを開始
        let mut start_usecase = StartGameUseCase::new(self.game_repository.clone());
        let params = StartGameParams {
            game_id: game_id.clone(),
        };
        if let Err(e) = start_usecase.execute(params) {
            GameView::display_error(&e);
            return;
        }
        
        // ゲームプレイループ
        self.game_play_loop(game_id);
    }
    
    fn select_game(&mut self) -> Option<GameId> {
        // 現在のゲームがあればそれを使用するか確認
        if let Some(current_id) = &self.current_game_id {
            if let Some(game) = self.game_repository.find_by_id(current_id) {
                println!("\n現在選択されているゲーム: {} ({})", game.id().value(), game.variant().name());
                println!("このゲームを使用しますか？ (y/n)");
                
                if InputHandler::get_bool("") {
                    return Some(current_id.clone());
                }
            }
        }
        
        // ゲーム一覧を表示して選択
        let games = self.game_repository.find_all();
        if games.is_empty() {
            GameView::display_error("ゲームが見つかりません。新しいゲームを作成してください。");
            return None;
        }
        
        println!("\n--ゲーム一覧--");
        for (i, game) in games.iter().enumerate() {
            println!(
                "{}. {} - {} (プレイヤー数: {})",
                i + 1,
                game.id().value(),
                game.variant().name(),
                game.players().len()
            );
        }
        
        let choice = match InputHandler::get_menu_choice(games.len()) {
            Ok(choice) => choice - 1,
            Err(e) => {
                GameView::display_error(&e);
                return None;
            }
        };
        
        let game_id = games[choice].id().clone();
        self.current_game_id = Some(game_id.clone());
        Some(game_id)
    }
    
    fn view_games(&self) {
        let games = self.game_repository.find_all();
        if games.is_empty() {
            println!("\nゲームはありません。");
            return;
        }
        
        println!("\n--ゲーム一覧--");
        for (i, game) in games.iter().enumerate() {
            println!(
                "{}. {} - {} (フェーズ: {}, プレイヤー数: {})",
                i + 1,
                game.id().value(),
                game.variant().name(),
                match game.current_phase() {
                    GamePhase::NotStarted => "準備中",
                    GamePhase::Dealing => "配り中",
                    GamePhase::Betting => "ベッティング",
                    GamePhase::Drawing => "カード交換",
                    GamePhase::Showdown => "ショーダウン",
                    GamePhase::Complete => "終了",
                },
                game.players().len()
            );
        }
        
        InputHandler::wait_for_enter();
    }
    
    fn game_play_loop(&mut self, game_id: GameId) {
        let mut game_over = false;
        
        while !game_over {
            // ゲームの現在の状態を取得
            let game = match self.game_repository.find_by_id(&game_id) {
                Some(game) => game,
                None => {
                    GameView::display_error("ゲームが見つかりません");
                    return;
                }
            };
            
            // ゲーム情報の表示
            GameView::display_game_info(&game);
            GameView::display_players(&game);
            GameView::display_community_cards(&game);
            
            // フェーズに応じた処理
            match game.current_phase() {
                GamePhase::Betting => {
                    self.handle_betting_phase(&game_id);
                },
                GamePhase::Drawing => {
                    self.handle_drawing_phase(&game_id);
                },
                GamePhase::Showdown => {
                    self.handle_showdown_phase(&game_id);
                    game_over = true;
                },
                GamePhase::Complete => {
                    println!("ゲームが終了しました。");
                    game_over = true;
                },
                _ => {
                    GameView::display_error("このフェーズはまだ実装されていません");
                    game_over = true;
                }
            }
            
            // ショートブレイク
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        
        println!("\n新しいゲームを始めますか？ (y/n)");
        if InputHandler::get_bool("") {
            let mut restart_usecase = StartGameUseCase::new(self.game_repository.clone());
            
            // ゲームをリセット
            if let Some(mut game) = self.game_repository.find_by_id(&game_id) {
                if let Err(e) = game.reset_for_new_hand() {
                    GameView::display_error(&e);
                    return;
                }
                
                if let Err(e) = self.game_repository.save(&game) {
                    GameView::display_error(&e);
                    return;
                }
                
                // ゲームを再開始
                let params = StartGameParams {
                    game_id: game_id.clone(),
                };
                if let Err(e) = restart_usecase.execute(params) {
                    GameView::display_error(&e);
                    return;
                }
                
                self.game_play_loop(game_id);
            }
        }
    }
    
    fn handle_betting_phase(&mut self, game_id: &GameId) {
        let game = match self.game_repository.find_by_id(game_id) {
            Some(game) => game,
            None => return,
        };
        
        // 現在のプレイヤーを取得
        let current_index = game.current_player_index();
        if current_index >= game.players().len() {
            GameView::display_error("有効なプレイヤーがいません");
            return;
        }
        
        let current_player = &game.players()[current_index];
        println!("\n現在のプレイヤー: {}", current_player.name());
        
        // プレイヤーが自分の手札を確認
        GameView::display_player_hand(current_player);
        
        // アクションの選択
        let (action, bet_amount) = match GameView::get_player_action(&game, current_index) {
            Ok(result) => result,
            Err(e) => {
                GameView::display_error(&e);
                return;
            }
        };
        
        // アクションを実行
        let params = PlaceBetParams {
            game_id: game_id.clone(),
            player_id: current_player.id().clone(),
            action,
            bet_amount,
        };
        
        let mut usecase = PlaceBetUseCase::new(self.game_repository.clone());
        if let Err(e) = usecase.execute(params) {
            GameView::display_error(&e);
        }
    }
    
    fn handle_drawing_phase(&mut self, game_id: &GameId) {
        let game = match self.game_repository.find_by_id(game_id) {
            Some(game) => game,
            None => return,
        };
        
        // 現在のプレイヤーを取得
        let current_index = game.current_player_index();
        if current_index >= game.players().len() {
            GameView::display_error("有効なプレイヤーがいません");
            return;
        }
        
        let current_player = &game.players()[current_index];
        println!("\n現在のプレイヤー: {}", current_player.name());
        
        // カード交換の選択
        let card_indices = match GameView::get_card_exchange(current_player) {
            Ok(indices) => indices,
            Err(e) => {
                GameView::display_error(&e);
                return;
            }
        };
        
        // カード交換を実行
        let params = ExchangeCardsParams {
            game_id: game_id.clone(),
            player_id: current_player.id().clone(),
            card_indices,
        };
        
        let mut usecase = ExchangeCardsUseCase::new(self.game_repository.clone());
        if let Err(e) = usecase.execute(params) {
            GameView::display_error(&e);
        }
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
            GameView::display_error(&e);
        }
        
        InputHandler::wait_for_enter();
    }
} 
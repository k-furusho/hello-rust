use std::path::Path;
use simple_poker::application::usecase::create_game_usecase::{CreateGameParams, CreateGameUseCase};
use simple_poker::domain::model::game::GameVariant;
use simple_poker::domain::repository::game_repository::GameRepository;
use simple_poker::domain::repository::player_repository::PlayerRepository;
use simple_poker::infrastructure::repository::file::game_repository_file::FileGameRepository;
use simple_poker::infrastructure::repository::inmemory::game_repository_inmemory::InMemoryGameRepository;
use simple_poker::infrastructure::repository::inmemory::player_repository_inmemory::InMemoryPlayerRepository;
use simple_poker::presentation::cli::menu::MenuController;
use std::env;

fn main() {
    println!("\n====================================");
    println!("      ポーカーゲーム DDD版         ");
    println!("====================================\n");
    
    // コマンドライン引数でストレージタイプを指定
    let args: Vec<String> = env::args().collect();
    let storage_type = if args.len() > 1 && args[1] == "file" {
        StorageType::File
    } else {
        StorageType::InMemory
    };
    
    // リポジトリの初期化
    match storage_type {
        StorageType::InMemory => {
            println!("インメモリストレージを使用します。");
            let game_repo = InMemoryGameRepository::new();
            let player_repo = InMemoryPlayerRepository::new();
            
            // デモゲームを作成（オプション）
            if let Err(e) = create_demo_game(&mut game_repo.clone()) {
                eprintln!("デモゲーム作成エラー: {}", e);
            }
            
            // メニューコントローラの作成と実行
            let mut menu = MenuController::new(game_repo, player_repo);
            menu.run();
        },
        StorageType::File => {
            println!("ファイルストレージを使用します。");
            match FileGameRepository::new(Path::new("data/games")) {
                Ok(game_repo) => {
                    let player_repo = InMemoryPlayerRepository::new();
                    
                    // メニューコントローラの作成と実行
                    let mut menu = MenuController::new(game_repo, player_repo);
                    menu.run();
                },
                Err(e) => {
                    eprintln!("ファイルストレージの初期化に失敗しました: {}", e);
                    eprintln!("インメモリストレージを代わりに使用します。");
                    
                    let game_repo = InMemoryGameRepository::new();
                    let player_repo = InMemoryPlayerRepository::new();
                    
                    // メニューコントローラの作成と実行
                    let mut menu = MenuController::new(game_repo, player_repo);
                    menu.run();
                }
            }
        }
    }
}

// ストレージタイプの列挙型
enum StorageType {
    InMemory,
    File,
}

// デモゲームを作成する関数
fn create_demo_game<R: GameRepository + Clone>(repo: &mut R) -> Result<(), String> {
    let params = CreateGameParams {
        variant: GameVariant::FiveCardDraw,
        small_blind: 5,
        big_blind: 10,
    };
    
    let mut usecase = CreateGameUseCase::new(repo.clone());
    usecase.execute(params)?;
    
    Ok(())
}
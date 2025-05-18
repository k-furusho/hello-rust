use std::io::{self, Write};

pub struct InputHandler;

impl InputHandler {
    pub fn get_string(prompt: &str) -> String {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力の読み取りに失敗しました");
        
        input.trim().to_string()
    }
    
    pub fn get_u32(prompt: &str) -> Result<u32, String> {
        let input = Self::get_string(prompt);
        input.parse::<u32>().map_err(|_| format!("無効な数値です: {}", input))
    }
    
    pub fn get_usize(prompt: &str) -> Result<usize, String> {
        let input = Self::get_string(prompt);
        input.parse::<usize>().map_err(|_| format!("無効な数値です: {}", input))
    }
    
    pub fn get_bool(prompt: &str) -> bool {
        let input = Self::get_string(prompt).to_lowercase();
        matches!(input.as_str(), "y" | "yes" | "はい" | "1")
    }
    
    pub fn get_menu_choice(max: usize) -> Result<usize, String> {
        let choice = Self::get_usize("選択")?;
        if choice == 0 || choice > max {
            return Err(format!("1から{}までの数字を入力してください", max));
        }
        Ok(choice)
    }
    
    pub fn get_card_indices() -> Result<Vec<usize>, String> {
        let input = Self::get_string("交換するカードの番号（スペース区切り、例: 1 3 5）");
        
        let indices: Result<Vec<usize>, _> = input
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect();
            
        indices.map_err(|_| "無効な番号が含まれています".to_string())
    }
    
    pub fn wait_for_enter() {
        print!("続けるにはEnterキーを押してください...");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力の読み取りに失敗しました");
    }
} 
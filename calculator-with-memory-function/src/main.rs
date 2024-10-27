use std::collections::{hash_map::Entry, HashMap};
use std::io::stdin;

struct Memory {
    slots: HashMap<String, f64>,
}

impl Memory {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    fn add_and_print(&mut self, token: &str, prev_result: f64) {
        let slot_name = token[3..token.len() - 1].to_string(); // & を削除
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                // メモリが見つかったので、値を更新・表示して終了
                *entry.get_mut() += prev_result;
                print_value(*entry.get());
            }
            Entry::Vacant(entry) => {
                // メモリが見つからなかったので、最後の要素に追加する
                entry.insert(prev_result);
                print_value(prev_result);
            }
        }
    }

    fn eval_token(&self, token: &str) -> Result<f64, String> {
        if let Some(slot_name) = token.strip_prefix("mem") {
            Ok(self.slots.get(slot_name).copied().unwrap_or(0.0))
        } else {
            token
                .parse()
                .map_err(|_| "Invalid number format".to_string())
        }
    }

    fn update_memory(&mut self, token: &str, value: f64) {
        let slot_name = &token[3..token.len() - 1].to_string();
        match self.slots.entry(slot_name.clone()) {
            Entry::Occupied(mut entry) => *entry.get_mut() += value,
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }
    }

    fn validate_slot_name(token: &str) -> Result<String, String> {
        let slot_name = token[3..token.len() - 1].to_string();
        if slot_name.is_empty() {
            return Err("Memory slot name cannot be empty".to_string());
        }
        Ok(slot_name)
    }
}

fn calculate(left: f64, right: f64, operator: &str) -> Result<f64, String> {
    match operator {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(left / right)
            }
        }
        _ => Err("Invalid operator".to_string()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut memory = Memory::new();
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        if line.is_empty() {
            break;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        // メモリ操作の処理
        if tokens[0].starts_with("mem") && (tokens[0].ends_with("+") || tokens[0].ends_with("-")) {
            let value = if tokens[0].ends_with("+") {
                prev_result
            } else {
                -prev_result
            };
            memory.update_memory(tokens[0], value);
            print_value(memory.eval_token(&tokens[0][..tokens[0].len() - 1])?);
            continue;
        }

        // 式の計算
        let left = memory.eval_token(tokens[0])?;
        let right = memory.eval_token(tokens[2])?;
        let result = calculate(left, right, tokens[1])?;
        print_value(result);

        prev_result = result;
    }

    Ok(())
}

fn print_value(value: f64) {
    println!("=> {}", value)
}

use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        // 1行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // 式の計算
        let left: f64 = tokens[0].parse().unwrap();
        let right: f64 = tokens[2].parse().unwrap();
        let result = calculate(left, right, tokens[1]);
        // 結果の表示
        print_value(result);
    }
}

fn print_value(value: f64) {
    println!("=> {}", value)
}

fn calculate(left: f64, right: f64, operator: &str) -> f64 {
    match operator {
        "+" => add_values(left, right),
        "-" => subtract_values(left, right),
        "*" => multiply_values(left, right),
        "/" => divide_values(left, right),
        _ => unreachable!(),
    }
}

fn add_values(left: f64, right: f64) -> f64 {
    left + right
}

fn subtract_values(left: f64, right: f64) -> f64 {
    left - right
}

fn multiply_values(left: f64, right: f64) -> f64 {
    left * right
}

fn divide_values(left: f64, right: f64) -> f64 {
    left / right
}

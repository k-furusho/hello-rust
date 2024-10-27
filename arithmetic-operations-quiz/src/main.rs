use rand::Rng;
use std::io;

fn main() {
    let mut num_of_correct = 0; // 正解数を数える変数
    while num_of_correct < 3 {
        // 正解数が3問以下の間は繰り返し
        if ask_question("+", |a, b| a + b) {
            num_of_correct += 1;
        }
        if num_of_correct >= 3 {
            break; // 3問正解したらループを抜ける
        }
        if ask_question("-", |a, b| a - b) {
            num_of_correct += 1;
        }
    }
}

fn ask_question<F>(operator: &str, operation: F) -> bool
where
    F: Fn(i32, i32) -> i32,
{
    let op1 = rand::thread_rng().gen_range(0..100);
    let op2 = rand::thread_rng().gen_range(0..100);
    println!("{} {} {} = ??", op1, operator, op2);
    println!("?? の値を入力してください:");
    let mut ans_input = String::new();
    io::stdin().read_line(&mut ans_input).unwrap();
    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    if dbg!(ans_input == operation(op1, op2)) {
        println!("正解！");
        true
    } else {
        println!("不正解！");
        false
    }
}

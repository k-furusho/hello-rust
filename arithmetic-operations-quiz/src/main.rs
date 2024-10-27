use rand::Rng;
use std::io;

fn main() {
    let mut num_of_correct = 0;
    while num_of_correct < 3 {
        let quiz_mode = rand::thread_rng().gen_range(1..=2); // 1か2をランダムに選択
        match quiz_mode {
            1 => {
                // quiz_modeが1のときは加算クイズ
                if ask_question("+", |a, b| a + b) {
                    num_of_correct += 1;
                }
            }
            2 => {
                // quiz_modeが2のときは減算クイズ
                if ask_question("-", |a, b| a - b) {
                    num_of_correct += 1;
                }
            }
            _ => unreachable!(),
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

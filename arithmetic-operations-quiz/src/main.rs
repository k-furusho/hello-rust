use rand::Rng;

fn main() {
    let mut num_of_correct = 0; // 正解数を数える変数
    while num_of_correct < 3 {
        // 正解数が3問以下の間は繰り返し
        let op1 = rand::thread_rng().gen_range(0..100);
        let op2 = rand::thread_rng().gen_range(0..100);
        println!("{} + {} = ??", op1, op2);
        println!("?? の値を入力してください:");
        let mut ans_input = String::new(); // ユーザーからの回答を保持する変数
        std::io::stdin().read_line(&mut ans_input).unwrap(); // 標準入力から1行取得し、ans_input に代入する
                                                             // ans_inputから trim()で改行を取り除き parse()で整数(i32)型に変換する
        let ans_input = ans_input.trim().parse::<i32>().unwrap();
        if dbg!(ans_input == op1 + op2) {
            println!("正解！");
            num_of_correct += 1; // 正解したら正解数を1増やす
            if num_of_correct >= 3 {
                break; // 3問正解したらループを抜ける
            }
        } else {
            println!("不正解！")
        }

        println!("{} - {} = ??", op1, op2);
        println!("?? の値を入力してください:");
        let mut ans_input = String::new();
        std::io::stdin().read_line(&mut ans_input).unwrap();
        let ans_input = ans_input.trim().parse::<i32>().unwrap();
        if dbg!(ans_input == op1 - op2) {
            println!("正解！");
            num_of_correct += 1;
        } else {
            println!("不正解！")
        }
    }
}

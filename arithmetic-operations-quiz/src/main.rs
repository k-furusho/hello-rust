fn main() {
    println!("1 + 1 = ??");
    println!("?? の値を入力してください:");
    // ユーザーからの回答を保持する変数
    let mut ans_input = String::new();

    // 標準入力から1行取得し、ans_input に代入する
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_inputから trim()で改行を取り除き parse()で整数(u32)型に変換する
    let ans_input = ans_input.trim().parse::<u32>().unwrap();
    dbg!(ans_input); // => cargo runした後にキーボードで入力した値が確認できる
    if dbg!(ans_input == 1 + 1) {
        println!("正解！");
    } else {
        println!("不正解！")
    }
}

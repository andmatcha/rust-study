use std::io;

fn main() {
    println!("数を予想しよう！");

    println!("キミの予想を入力してねん");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("行の読み込みに失敗しました");

    println!("キミの予想: {}", guess);
}

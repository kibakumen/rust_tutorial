use std::io;
fn main(){
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);

    println!("数字を予想してください！");

    println!("予想を書き込もう！");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("行の読み込みに失敗しました");

    println!("あなたの予想：{guess}");   
}


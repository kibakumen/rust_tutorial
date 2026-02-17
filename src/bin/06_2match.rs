fn main() {
    //matchによるコイン数え上げ装置
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) ->u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => { //複数行のコードを書く時は{}が必要だが、後ろの,は省略できる
                println!("Lucky dime!");
                1
            }
            Coin::Quarter =>25,
        }
    }
    

    //列挙子の中にデータを保持させる
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --略--
    }

    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }


    //列挙子による関数適用
    fn value_in_cents2(coin: Coin2) -> u8 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("{:?}州のクォーター", state);
                25
            }
        }
    }

    value_in_cents2(Coin2::Quarter(UsState::Alaska));

    
    //Option型の活用
    fn plus_one(x: Option<i32>) -> Option<i32>{
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!(
        "Some(5):{:?}\n
        plus_one(five):{:?}\n
        plus_one(None):{:?}",
        five,
        six,
        none,
    );

    println!("\n===============");

    //catch-allパターン
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //最後に例外処理
    }

    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}
    fn move_player(num_spaces: u8){}

    //値を束縛しないcatch-all
    let dice_roll2 = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), //_がワイルドカードで、()が何もしないユニット値
    }
}
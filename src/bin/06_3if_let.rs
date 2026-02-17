fn main() {
    //一つのパターンの処理だけ書きたい場合
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("最大値は{}に設定されます",max),
        _ => (),
    }

    //通常のmatchだと二行必要で冗長

    //if letの使用
    let config_max_iflet = Some(3u8);
    if let Some(max) = config_max { //matchの糖衣構文
        println!("最大値は{}に設定されます", max);
    }

    //簡潔になるがmatchの包括性は失うトレードオフ

    //一応elseでcatch-allパターンもできる
    
    enum Coin {
        Quarter(UsState),
        Anywhere
    }
    
    #[derive(Debug)]
    enum UsState {
        Alaska
    }

    fn count_coin (coin: Coin){
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
            println!("カウントが増えました");
        }
    }

    count_coin(Coin::Quarter(UsState::Alaska));
    println!("\n================");

    count_coin(Coin::Anywhere);

    
}
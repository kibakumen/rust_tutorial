fn main(){
    use std::collections::HashMap;

    //作成
    let mut scores = HashMap::new();
        //挿入
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20); //値とキー、それぞれの型が一致している必要がある

    //取得

        //getメソッド
    let team_name = String::from("Blue");
    let Blue_score = scores.get(&team_name).copied().unwrap_or(0);//getメソッドはOption<&V>を返す
    println!("{Blue_score}");

    println!("\n====================");

        //for文を使った走査
    for (key, value) in &scores {
        println!("{key}:{value}");
    }
    println!("\n======================");

    //ハッシュマップに挿入された参照型はその時点で所有権をムーブされて無効になる
    //所有権をムーブしたくなかったら借用を使おう

    //更新

        //上書き
    scores.insert(String::from("Blue"), 25); //そのまま既存キーにinsertする
    println!("上書きしたBlueのスコア：{:?}", scores.get(&team_name));
    println!("\n======================");

       //キーが存在しない場合のみキーと値を追加する
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    //entryメソッドはEntry<'_, K, V>という存在する可能性のあるenumを返す
    //or_insertメソッドは対応するEntryキーが存在したときにそのキーに対する値への可変参照を返すために定義されている
    //もしなかったら、引数をこのキーの新しい値として挿入し、新しい値への可変参照を返す。

    println!("{:?}", scores);
    println!("\n======================");

        //古い値に基づいて値を更新する
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() { //split_whitespaceメソッドは、textの値から、ホワイトスペースによって区切られた部分スライスを走査するイテレータを返す
        let count = map.entry(word).or_insert(0);//初めてヒットしたときは0を挿入する。ヒット済みの場合は下で1増やす
        *count += 1;//参照外しをする
    }//ここで借用が終る


}
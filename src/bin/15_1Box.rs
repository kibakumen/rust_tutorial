// スマートポインタとは
    //通常の参照に追加のメタデータと能力を付与したデータ構造
    //参照と違い、指しているデータを所有していることが多い
    //DerefトレイトとDropトレイトを実装した構造体である
    //Vec<T>やStringもスマートポインタである

// Box<T>
    // 特別な能力を持たないシンプルなスマートポインタ


//作成
fn create_Box () {
    let b = Box::new(5);
    println!("b = {}", b);
}


//コンスリスト
    //Lisp由来の再帰的なデータ構造
    //(1, (2, (3, Nil)))
        //Nilは基底値
    
    //通常のenumなどで直接再帰値を扱うとサイズが無限になるためコンパイルできない
// enum List {
//     Cons(i32, List),
//     Nil
// }

    //なので、Box<T>で間接参照を用いて再帰値へのポインタの方を格納させる
enum BoxList {
    Cons(i32, Box<BoxList>), // サイズが有限 i32 + ポインタ 
    Nil,
}

use crate::BoxList::{Cons, Nil};

fn list_recursion() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
    

fn main() {
    create_Box();
    list_recursion();
}


// Derefトレイト
    //参照外し演算子 * のふるまいをカスタマイズする

// 参照外しの2つのレイヤー
    // 1. 明示的参照外し（*）: プログラマが自分で * を書く
    // 2. Deref Coercion（参照外し型強制）: コンパイラが自動で * を挿入する
    //    → 関数やメソッドの引数で型が合わないとき発動
    //    → 例: &Box<String> → &String → &str と自動チェーン変換

// なぜ独自の型はそのままでは * が使えないのか？
    // &T に対する * は言語組み込みのプリミティブ操作
    //   → コンパイラが「アドレスの先にTがある」と知っている
    // 独自の型はフィールドが複数ある可能性がある
    //   → * で「何を返すべきか」がコンパイラには判断できない
    //   → Rustは曖昧さを許容しない → Derefトレイトで明示する必要がある


//通常の参照外し
fn normal_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // &T → 言語プリミティブ。Derefトレイト不要
}


//Box<T>を参照のように使う
fn box_ref() {
    let x = 5;
    let y = Box::new(x); // xからコピーされた値のインスタンス

    assert_eq!(5, x);
    assert_eq!(5, *y); // 独自の型でもDerefトレイトがあるので参照外し演算子が使える
}

//Box<T>を自作してその構造を確かめる（ヒープ置く特性は無視する）
    //型構造を突き詰めると、一要素のタプル
struct MyBox<T>(T); 

    //new関数
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

    //このままでは *MyBox は使えない
    //Derefトレイトを実装して「*で返すべき値」を教える
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;          // *で返す型はT
    fn deref(&self) -> &Self::Target {   // 具体的な取り出し方を定義
        &self.0               // タプル構造体の最初の要素への参照
    }
}
    // *y を書くと、コンパイラは内部的に *(y.deref()) に展開する
    // deref()は&Tを返すので、最後の*は通常の参照外し（言語プリミティブ）

fn mybox_deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref()) → *(&5) → 5
}


// Deref Coercion（参照外し型強制）
    // 関数の引数で型が合わないとき、コンパイラが自動でderef()をチェーン呼び出し
    // &MyBox<String> → &String → &str のように自動変換される

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));

    // Deref Coercionのおかげで &MyBox<String> → &String → &str に自動変換
    hello(&m);

    // もしDeref Coercionがなかったら、こう書く必要がある
    // hello(&(*m)[..]);
    //   *m       → MyBox<String>からStringを取り出す
    //   [..]     → Stringから文字列スライスにする
    //   &        → 参照にする
}


// 参照外し型強制が可変性と相互作用する方法
    // Deref Coercionは可変性に応じて3つのパターンがある
    //
    // 1. &T → &U       （T: Deref<Target=U> のとき）
    //    不変参照 → 不変参照。上のderef_coercionで見たパターン
    //
    // 2. &mut T → &mut U （T: DerefMut<Target=U> のとき）
    //    可変参照 → 可変参照。DerefMutトレイトが必要
    //
    // 3. &mut T → &U    （T: Deref<Target=U> のとき）
    //    可変参照 → 不変参照。可変を不変に「格下げ」するのは安全なのでOK
    //
    // &T → &mut U は絶対にできない
    //    不変参照を可変参照に「格上げ」すると借用のルールが壊れる
    //    → 不変参照が存在する間は、他に不変参照があるかもしれない
    //    → それなのに可変にしたら、データ競合が起きる

use std::ops::DerefMut;

impl<T> DerefMut for MyBox<T> {
    // DerefMutはDerefを実装済みの型にしか実装できない
    // type Targetは不要（Derefで既に定義済み）
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello_mut(name: &mut String) {
    name.push_str("!!!"); 
    println!("Hello, {}!", name);
}

fn deref_mut_demo() {
    // パターン2: &mut MyBox<String> → &mut String
    let mut m = MyBox::new(String::from("Rust"));
    hello_mut(&mut m); // 可変Deref Coercionが働く

    // パターン3: &mut MyBox<String> → &String → &str（可変→不変の格下げ）
    let mut m2 = MyBox::new(String::from("World"));
    hello(&m2);        // 可変な値でも不変参照として渡せる
}


fn main() {
    normal_deref();
    box_ref();
    mybox_deref();
    deref_coercion();
    deref_mut_demo();
}
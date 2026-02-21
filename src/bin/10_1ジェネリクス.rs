//関数
use std::cmp::PartialOrd;// char型でも比較演算子が使えるようにするライブラリ

fn find_largest<T: PartialOrd>(list: &[T]) -> &T{ //関数名の後ろ型名宣言をする
    let mut largest = &list[0]; 
    for acc in list{
        if acc > largest {
            largest = acc;
        };
    }
    largest
}


//構造体
struct Point<T> {//<f32>のように型を制約することもできる
    x: T,
    y: T,
}

    //複数の型引数を使用
struct Point2<T, U> {
    a: T,
    b: U,
}

    //メソッド定義
impl<T> Point<T> {
    fn x(&self) -> &T {//xのゲッター
        &self.x
    }
}

    //メソッドで別の型を含める
impl<T, U> Point2<T, U> {
    fn mixup<X, Y>(self, other: Point2<X, Y>) -> Point2<T, Y>{
        Point2 {
            a: self.a,
            b: other.b,
        }
    }
}


//enum
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}


//


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = find_largest(&number_list);
    println!("最大値：{}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = find_largest(&char_list);
    println!("最大字：{}", result);

    println!("\n======================");

    let integer = Point {x: 5, y: 10};//型引数は一つなので両方同じ型にする
    let float = Point {x: 1.0, y: 4.0};
    println!("integer.x = {}", integer.x());

    let both_integer = Point2 { a: 5, b: 10 };
    let int_float = Point2 { a: 5, b: 4.0 };

}
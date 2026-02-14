fn main(){
    rectangles();
    println!("\n===================");

    rec_tup();
    println!("\n===================");

    rec_struct();
    println!("\n===================");

    struct_debug();


}

//通常の関数適用によるプログラム
fn rectangles(){
    let width1 = 30;
    let height1 = 50;

    println!(
        "長方形の面積は、{}平方ピクセルです",
        area1(width1, height1)
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

//area関数の引数に関連性があることが明確になっていない

//タプルを使用
fn rec_tup() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//渡す引数が一つになり、構造化されたが、要素の明確性がない
    //データの意味がコードに載っていない

//構造体を使用
struct Rectangle { 
    width: u32,
    height: u32,
}

fn rec_struct(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "長方形の面性は{}平方ピクセルです",
        area3(&rect1)//rec_structがrect1を継続利用できるように借用にしている
    )
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height//借用された構造体インスタンスのフィールドにアクセスしても、そのフィールドの値はムーブされない
}

//構造体インスタンスを出力
#[derive(Debug)]//デバッグ属性を構造体に付与
struct Rectangle_debug {
    width: u32,
    height: u32,
}

fn struct_debug() {
    let rect1 = Rectangle_debug {
        width: 30,
        height: 50,
    };

    println!("rect1は{:?}です", rect1);//{:?}でデバッグ出力
    println!("rect1は{:#?}です", rect1);//{:#?}で整形してくれる
}

//ほかにもdbg!(&rect1);のようにデバッグ用のマクロを呼ぶ方法もある。
    //このマクロは所有権を奪うので、借用にしている
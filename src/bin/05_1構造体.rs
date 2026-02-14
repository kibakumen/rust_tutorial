fn main() {
    //構造体の定義
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    //インスタンス化
    let user1 = User{
        active: true,
        username: String::from("anonymous"),
        email: String::from("anonymous@example.com"),
        sign_in_count: 1,
    };

    //ドット記法で値取得
    let email = user1.email;
    println!("{email}");


    //可変にする場合はインスタンス全体を可変にする必要がある
    let mut user2 = User{ //ここでmutをつけて可変にしている
        active: true,
        username: String::from("anonymous_mut"),
        email: String::from("anonymous_mut@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");
    println!("{}", user2.email);

    println!("\n===================");

    //インスタンス生成関数
    fn _build_user(email: String, username: String) -> User{
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    //初期化省略記法
    fn _build_user2(email: String, username: String) -> User {
        User {
            active: true,
            username, //フィールド名と仮引数が同じなら重複して書かなくてもいい
            email, //同様
            sign_in_count: 1,
        }
    }

    //構造体更新記法
    let _user3 = User {
        email: String::from("updated@example.com"),
        ..user1 //更新箇所以外を展開。最後に記述する
    };

    //user1のヒープ領域を使用するフィールド（username）などから所有権をムーブしているため、user1はそのまま使えなくなる。
    
    //タプル構造体
    struct Color(i32, i32, i32);//フィールド名がなく、型だけを定義
    struct Point(i32, i32, i32);

    fn _tup_struct() {
        let _black = Color(0, 0, 0);
        let _origin = Point(0, 0, 0);
    }

    //ユニット様構造体
    struct AlwaysEqual;//()や{}は不要

    fn _unit_struct(){
        let _subject = AlwaysEqual;
    }

}
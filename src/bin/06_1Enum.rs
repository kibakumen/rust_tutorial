fn main(){
    //列挙型の定義
    enum IpAddrKind {
        V4,
        V6,
    }

        //インスタンス
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

        //仮引数の定義
    fn _route(_ip_kind: IpAddrKind){}


    //構造体との組み合わせ
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    //enumだけで同じ概念を表現
        //こちらの方が簡潔
    enum IpAddrEnum {
        V4(u8, u8, u8, u8),//片方のデータ属性を別のものに変えられる
        V6(String),
    }

    let _home = IpAddrEnum::V4(127, 0, 0, 1);

    let _loopback = IpAddrEnum::V6(String::from("::1"));


    //実際のIPアドレスの定義方法
    struct Ipv4Addr {
        // --略--
    }
    
    struct Ipv6Addr {
        // --略--
    }
    
        //列挙子内で構造体を含めたいかなるデータも格納できる
    enum IpAddrReal {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    
    //多様な種類のデータ型を含むenum
    enum Message {
        Quit,                         //なし
        Move {x: i32, y: i32},        //名前付きフィールド
        Write(String),                //オブジェクトタプル
        CharengeColor(i32, i32, i32), //複数の数値タプル
    }


    //メソッド定義
    impl Message {
        fn call(&self) {
            
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    //Option
        //null値問題の対抗策として実装されている標準ライブラリ
    enum Option<T> {
        None,
        Some(T),
    }
        //列挙子もOption::の接頭辞なしに直接使える
    
    let some_number = Some(5);
    let some_char = Some('e');

    //None値を見ただけでは、それに対応するSome列挙子が保持する型をコンパイラは推論できない
    //そのため、コンパイラはジェネリクス適用後のOption型を注釈することを要求する
    // let absent_number: Option<i32> = None
        

}
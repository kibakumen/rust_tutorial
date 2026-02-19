fn main(){
    //エラーには二種類ある
        //回復不能なエラー：panic!マクロ
        //回復可能なエラー：Result<T, E>


    //panic!マクロ
        //失敗メッセージを出力し、スタックを巻き戻して片付け、プログラムを終了させる
    panic!("========\ncrash!\n========");    

    //バックトレースを遡ってエラーの過程を分析できる


    //Result型
    enum _Result<T, E> {
        Ok(T),
        Err(E),
    }
    
    //戻り値がResultな関数の使用
    use std::fs::File;
    let greeting_file_result = File::open("hello.txt") //File::openはResult<T, E>を返す

    //返される三つの情報
        //成功したか失敗したか
        //成功値の型：std::fs::Fileで埋められたファイルハンドリングによるOkインスタンス
        //エラー値の型：std::io::ErrorによるErrインスタンス
    

    //match文の使用
    use std::io::ErrorKind; //io処理の結果発生する可能性のある色々な種類のエラーを表す
    let greeting_file = match greeting_file_result {
        Ok(file) => file, //Resultの列挙子は標準化されているのでResult::をしなくてもいい

        //内側にmatch式を追加し、エラーごとの対応を細分化
        Err(error) => match error.kind(){ //.kind：呼び出すとio::ErrorKind値が得られる
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("ファイルを作成するのに問題があります：{:?}", error),
            },
            other_error => {
                panic!("ファイルを開くのに問題がありました：{:?}", other_error);
            }
        },
    };


    //unwrap
    let bye_file = File::open("bye.txt").unwrap();
        //成功時：Okの中身を返す
        //失敗時：panic!マクロを呼ぶ

    //expect
    let expect_file = File::open("expect.txt")
    .expect("expect.txtが含まれるべきです");
        //失敗時のpanic!マクロのメッセージ内容を指定できる
    

    //エラーの委譲
    //発生個所で処理せず、呼び出し元がどうするか決められるようにエラーを返す
    use std::io::{self, Read};
    
    fn read_username_from_file() -> Result<String, io::Error> {
        //該当ファイルを開く
        let username_file_result = File::open("name.txt");

        //開こうとした結果のパターン
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),//panic!を呼び出す代わりに、File::openから得たエラー値を早期に返す
        };

        //書き込み先の文字列型を初期化
        let mut username = String::new();

        //ファイルの中身を読み込み、usernameに書き込もうとしたときのパターン
        match username_file.read_to_string(&mut username) { //読みだし内容を書き込むので可変参照にしている
            Ok(_) => Ok(username),
            Err(e) => Err(e),//最後の戻り値になるので明示的にreturnをつける必要はない
        }
    }
    //返したOk値やErr値をどう扱うかは関数の呼び出し元に任せる


    // ? 演算子
        //Result値を返す処理の末尾に?を置き、成功したらOk値を返してプログラムを継続、失敗したらErr値を返させる
        //match式との違い：型変換を行うfrom関数を通り、受け取ったエラー型をシグネチャの戻り値部分のエラー型に変換させる

    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("name.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

        //メソッドチェーンによる省略
    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username = String::new();
        //処理を一度に済ませられるのでusername_file変数の中継がいらない
        File::open("hello.txt")?.read_to_string(&mut username)?;


        Ok(username)
    }

    //注意点として、?演算子を使用する際は関数シグネチャで戻り値型をResult（またはOption,FromResidual）にし、演算子の想定と互換性を持たせる必要がある
    //またResultやOptionの?演算子を同一関数内で混ぜて使用することはできない

        //なお、上記はよくある操作なので標準ライブラリfsで定義されている
    use std::fs;

    fn read_username_from_file4() -> Result<String, io::Error> {
        fs::read_to_string("name.txt")
    }


    //エラーハンドリング方法の判断基準

        //panic!すべきか
        //panic!するとその時点で回復不可能という決定を下すことになり、プログラムを中断することになる
        //継続する余地があるならResultを使おう
}
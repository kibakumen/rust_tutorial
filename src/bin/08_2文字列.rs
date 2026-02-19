// ==========================================
// 8-2: 文字列でUTF-8エンコードされたテキストを保持する
// ==========================================
//
// ■ なぜ文字列は難しいのか？（3つの理由）
//   1. Rustがエラーを隠さない性質
//      → 他の言語がこっそり許している不正な操作をコンパイル時に弾く
//   2. 文字列は見た目以上に複雑なデータ構造
//      → 「1文字」の定義自体が曖昧（バイト？スカラー値？書記素クラスタ？）
//   3. UTF-8エンコーディング
//      → 1文字のバイト数が1〜4バイトで可変（ASCIIの固定1バイトとは違う）
//
// ■ Rustの2つの文字列型
//
//   str（文字列スライス）
//   ├─ 言語のコアに組み込まれた唯一の文字列型
//   ├─ 通常は借用形態 &str で使う
//   ├─ 別の場所に格納されたUTF-8データへの「参照」
//   └─ 文字列リテラル "hello" はプログラムのバイナリに格納 → &str
//
//   String（文字列）
//   ├─ 標準ライブラリが提供する型（言語コアではない）
//   ├─ 伸長可能・可変・所有権あり
//   ├─ 内部的には Vec<u8> のラッパー
//   └─ UTF-8エンコード
//
//   → どちらもUTF-8。Rustaceanが「文字列」と言うとき、
//     String か &str のどちらかを指している
//
// ■ なぜ s[0] で添え字アクセスできないのか？
//
//   例: "Hola" → 4バイト（各文字1バイト）... len() == 4 ← 直感通り
//   例: "Здравствуйте" → 24バイト（各キリル文字2バイト）... len() == 24 ← 12文字なのに！
//   例: "こんにちは" → 15バイト（各日本語文字3バイト）... len() == 15
//
//   s[0] が返すのは「最初のバイト」であり「最初の文字」ではない
//   → "Здравствуйте"[0] は З(=208,151) の 208 を返すことになる
//   → 208は単独では有効な文字ではない → バグの原因になる
//   → だからRustはコンパイル時に禁止する
//
// ■ 文字列を見る3つのレンズ（ヒンディー語 "नमस्ते" の例）
//
//   1. バイト（bytes）: [224, 164, 168, 224, 164, 174, ...] → 18バイト
//   2. Unicodeスカラー値（chars）: ['न', 'म', 'स', '्', 'त', 'े'] → 6つ
//   3. 書記素クラスタ: ["न", "म", "स्", "ते"] → 人間が認識する4文字
//
//   → どの「見方」が正しいかは用途次第！
//   → Rustは chars() や bytes() でプログラマに選択させる
//   → 添え字アクセスは O(1) を保証できない（先頭から走査が必要）ので禁止
//

fn main() {
    //作成
        //Vec<u8>由来のnew関数
    let mut s_new = String::new(); //空の文字列を作成

        //from関数
    let mut s_from = String::from("initial contents");

        //to_stringメソッド
    let s_method = "initial contents".to_string(); //from関数と同じ操作


    //更新
        //push_str
    let mut s_foo = String::from("foo");
    let s_bar = "bar";

    s_foo.push_str(s_bar); //push_strメソッドは引数を不変借用する
    println!("s_fooは{s_foo}になり、s_barは{s_bar}です"); //push_strはs_barの所有権を求めないので、以降も使用できる
    //pushメソッドはchar型を引数にとる
    println!("\n======================");

        //+演算子
    let s_plus_1 = String::from("Hello, ");
    let s_plus_2 = String::from("world!");
    let s_plus_1and2 = s_plus_1 + &s_plus_2; //後方の要素を借用して前方をムーブしたものに結合させる
        //Stringでの+演算子のメソッドシグネチャ：fn add(self, s:&str) -> String{～
        //上記の&s_plus_2は&strではなく、&Stringになるはずなので、順当に考えればコンパイルエラーになる
        //しかし、コンパイラーが&String引数を&strに型強制してくれる。具体的には参照外し型強制によって&s_plus_2を&s_plus_2[..]に変えている
        //これらの操作により、fn add()の第二引数にとられている&str(s_plus_2)は不変借用されているだけなので、以降も使用できる
        //一方で第一引数にとられているself（s_plus_1）は借用ではなく、所有権をムーブされるので、以降は無効になる

    //+演算子は二つの引数をとることを前提にしているので、三つ以上の連結では扱いが複雑になる

        //format!マクロ
    let s_tic = String::from("tic");
    let s_tac = String::from("tac");
    let s_toe = String::from("toe");

    let tic_tac_toe = format!("{s_tic}-{s_tac}-{s_toe}");//三つ以上の文字列の連結なら＋演算子よりもわかりやすい
    println!("{tic_tac_toe}");
    println!("\n======================");


    //添え字アクセスできない理由の実験
    let s_indexed = String::from("hello");
    // let h = s_indexed[0]; ←エラーになる
    //Unicodeのスカラー値と添え字アクセスで返されるバイトが相互に対応しておらず混乱を招くため
        //他の言語ではアクセスできたりする。混乱するのを許容するなら


    // 文字列のスライス
    let hello = "Здравствуйте";

    println!("&hello[0..4]：{:?}", &hello[0..4]);//特定のバイトを含む文字列スライスを作る
    //これらの文字は各々2バイトになるので、&hello[0..4]はЗдになる


    // - 文字列の走査

        //Unicodeスカラー値として扱う場合(charsメソッド)
    println!("Unicodeスカラー値として扱う場合");
    for c in "Зд".chars(){
        println!("{c}");
    }

        //バイトとして扱う場合（bytesメソッド）
    println!("バイトとして扱う場合");
    for b in "Зд".bytes(){
        println!("{b}");
    }

    println!("\n======================");

    // ==========================================
    // 付録: 標準ライブラリの文字列メソッド群
    // ==========================================
    // Rustは添え字アクセスを禁止する代わりに、
    // UTF-8を正しく扱う豊富なメソッドを提供している

    // --- 検索系 ---
    let s = String::from("こんにちは、世界！");
    println!("contains: {}", s.contains("世界"));           // → true
    println!("starts_with: {}", s.starts_with("こんにちは")); // → true
    println!("ends_with: {}", s.ends_with("！"));            // → true
    println!("find: {:?}", s.find("世界"));                  // → Some(18)（バイト位置）
    println!("\n======================");

    // --- 変換・置換系 ---
    let s2 = "Hello, World!";
    println!("replace: {}", s2.replace("World", "Rust"));       // → "Hello, Rust!"
    println!("replacen: {}", s2.replacen("l", "L", 2));         // → "HeLLo, World!"
    println!("to_uppercase: {}", s2.to_uppercase());             // → "HELLO, WORLD!"
    println!("to_lowercase: {}", s2.to_lowercase());             // → "hello, world!"
    println!("\n======================");

    // --- トリム・分割系 ---
    let s3 = "  Hello, World!  ";
    println!("trim: [{}]", s3.trim());                           // → "Hello, World!"
    println!("trim_start: [{}]", s3.trim_start());               // → "Hello, World!  "
    println!("trim_end: [{}]", s3.trim_end());                   // → "  Hello, World!"

    let csv = "apple,banana,cherry";
    let fruits: Vec<&str> = csv.split(',').collect();
    println!("split: {:?}", fruits);                             // → ["apple", "banana", "cherry"]

    let lines = "line1\nline2\nline3";
    let v: Vec<&str> = lines.lines().collect();
    println!("lines: {:?}", v);                                  // → ["line1", "line2", "line3"]
    println!("\n======================");

    // --- 判定系 ---
    let s4 = "12345";
    println!("is_empty: {}", s4.is_empty());                         // → false
    println!("all numeric: {}", s4.chars().all(|c| c.is_numeric())); // → true
    println!("café is_ascii: {}", "café".chars().all(|c| c.is_ascii())); // → false（éがASCII外）
    println!("hello is_ascii: {}", "hello".chars().all(|c| c.is_ascii())); // → true
    println!("\n======================");

    // --- 結合・繰り返し ---
    let words = vec!["Rust", "is", "great"];
    let sentence = words.join(" ");
    println!("join: {}", sentence);          // → "Rust is great"

    let repeated = "Ha".repeat(3);
    println!("repeat: {}", repeated);        // → "HaHaHa"
    println!("\n======================");

    // --- 安全な文字切り詰め（Rustが正しさを強制する例）---
    // JavaScriptでは "Hi😀👋🎉"[0..5] で壊れた文字が混入するが、
    // Rustでは chars() を使って文字単位で安全に処理できる
    let emoji_str = String::from("Hi😀👋🎉");
    let truncated: String = emoji_str.chars().take(4).collect();
    println!("安全な切り詰め: {}", truncated); // → "Hi😀👋"（絵文字が壊れない）
    println!("元の文字列のバイト数: {}", emoji_str.len());       // バイト数
    println!("元の文字列の文字数: {}", emoji_str.chars().count()); // 文字数（コードポイント単位）

    println!("\n=== 8-2 文字列 完了 ===");
}

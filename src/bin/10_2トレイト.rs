//トレイト：型のふるまいをグループにまとめて共通化する
    //他の言語ではインターフェイスと呼ばれる
    //implのようにtraitキーワードを使用する
    //メソッドの実装（{}内の記述）は定義せず、シグネチャだけを定義できる

fn main() {
    //異なる文書構造体（ArticleやTweetなど）の要約を出力したい

    //トレイトの作成
    pub trait Summary {//これに依存するクレートに公開するためにpubをつける
        fn summarize(&self) -> String;//具体的な実装は個別のメソッドに任せる
    }

    //型に実装
    pub struct Article { 
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for Article { //forキーワードで適用
        fn summarize(&self) -> String { //規定のシグネチャを使用
            format!("{}, by {} ({})",
                self.headline,
                self.author,
                self.location,
            )
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    //ユースケース
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "新製品発表！！"
        ),
        reply: false,
        retweet: false,
    };

    println!("new tweet: {}", tweet.summarize());

    // 注意すべき制限の1つは、トレイトか、対象の型のうち、少なくとも一方が自分のクレートに固有(local)である時のみ、 型に対してトレイトを実装できるということです。
    // 例えば、Displayのような標準ライブラリのトレイトをaggregatorクレートの機能の一部として、 Tweetのような独自の型に実装できます。
    // 型Tweetがaggregatorクレートに固有だからです。 
    // また、SummaryをaggregatorクレートでVec<T>に対して実装することもできます。 
    // トレイトSummaryは、aggregatorクレートに固有だからです。
    // 外部のトレイトを外部の型に対して実装することはできません。


    //デフォルト実装
    pub trait Summary2 {
        fn summarize(&self) -> String {
            String::from("（もっと読む）")
        }
    }

    impl Summary2 for Article {}

    let
    
    println!(
        "新しい記事があります！ {}",
        article.summarize()
    )
}



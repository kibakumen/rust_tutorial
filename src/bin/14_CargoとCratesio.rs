// 14章: CargoとCrates.ioについてより詳しく
// 公式: https://doc.rust-jp.rs/book-ja/ch14-00-more-about-cargo.html


// ============================
// 14-1: リリースプロファイル
// ============================
// Cargoには2つの主なプロファイルがある
    // dev: cargo build で使われる（開発用）
    // release: cargo build --release で使われる（リリース用）

// Cargo.toml でカスタマイズできる
// [profile.dev]
// opt-level = 0     ← 最適化なし。コンパイル速い、実行遅い
//
// [profile.release]
// opt-level = 3     ← 最大最適化。コンパイル遅い、実行速い
//
// opt-level は 0〜3 の範囲
// デフォルトを上書きしたいときだけ Cargo.toml に書く


// ============================
// 14-2: Crates.ioにクレートを公開する
// ============================

// ドキュメンテーションコメント（///）
    // Markdown記法が使える
    // cargo doc --open でHTMLドキュメントが生成される
    // よく使うセクション: Examples, Panics, Errors, Safety
    
/// 与えられた数値に1を足す
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
    // ※ cargo test で /// 内のコードが「ドキュメントテスト」として実行される
    // → ドキュメントとコードがずれない仕組み

// クレート全体のコメント（//!）
    // ファイルの先頭に書く。クレート全体の説明

// pub use で再エクスポート
    // 内部のモジュール構造が深くても、ユーザーが使いやすいAPIを公開できる
    // 例: pub use self::kinds::PrimaryColor; 
    //   → my_crate::PrimaryColor でアクセスできるようになる

// 公開手順（将来使うとき用メモ）
    // 1. https://crates.io でGitHubアカウントでログイン
    // 2. APIトークンを取得
    // 3. cargo login <トークン>
    // 4. Cargo.toml に [package] メタデータを記入
    //    name, version, edition, description, license が必須
    // 5. cargo publish
    // ※ 一度公開したバージョンは削除不可能（上書きも不可）
    // ※ 非推奨化は cargo yank --vers <バージョン>


// ============================
// 14-3: Cargoのワークスペース
// ============================
// 複数の関連するパッケージを1つのプロジェクトで管理する仕組み
    // ルートの Cargo.toml に [workspace] セクションを定義
    // 全パッケージが1つの target/ と Cargo.lock を共有する
    //   → 依存クレートのバージョンが統一される

// 構成例:
// my_project/
// ├── Cargo.toml          ← [workspace] members = ["adder", "add_one"]
// ├── adder/              ← バイナリクレート
// │   ├── Cargo.toml
// │   └── src/main.rs
// └── add_one/            ← ライブラリクレート  
//     ├── Cargo.toml
//     └── src/lib.rs

// ワークスペース内で特定パッケージを実行: cargo run -p adder
// ワークスペース内で特定パッケージをテスト: cargo test -p add_one


// ============================
// 14-4: cargo installでバイナリをインストール
// ============================
// Crates.ioのバイナリクレートをローカルにインストールできる
    // cargo install ripgrep → ~/.cargo/bin/rg がインストールされる
    // 12章で触れたgrepのRust版（ripgrep）がまさにこの方法でインストール可能
    // バイナリターゲットを持つクレート（src/main.rsがあるもの）のみ対象


fn main() {
    println!("14章はCargo周りの知識。コードの実行はなし。");
    println!("add_one(5) = {}", add_one(5));
}

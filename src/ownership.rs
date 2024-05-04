pub fn run() {
    println!("ownership.rs is start!");

    let s1 = String::from("hello");
    println!("The value of s1 is: {}", s1);

    // 所有権者のみがヒープ解放(drop)
    // 二重解放エラー（先に解放があり、整合性が取れない）を防ぐ
    // NOTE: Rustが自動で設定
    // String型はRustの所有権が適用される
    let s2 = s1;
    println!("The value of s2 is: {}", s2);

    // 文字列スライスは所有ではなく参照
    // 文字列リテラルから文字列スライス作成
    let s3 = "hello";
    // 静的領域にあるため解放不要
    println!("The value of s3 is: {}", s3);

    // String型から文字列スライス作成
    let s4 = String::from("hello");
    // &s4: 参照取得
    let s5 = &s4;
    // 所有権はs4(String型)から移動しない
    println!("The value of s5 is: {}", s5);

    println!("ownership.rs is done!\n------");
}

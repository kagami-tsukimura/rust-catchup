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

    println!("");
    println!("ownership.rs is done!\n------");
}

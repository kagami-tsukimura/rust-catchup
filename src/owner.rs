pub fn run() {
    println!("owner.rs is start!");

    // 所有権：s1
    let s1 = String::from("hello");
    // 所有権：s1 → s2
    let s2 = s1;
    // s1に所有権がない（s2にmove）ためエラー
    // println!("{}, {}", s1, s2);

    println!("owner.rs is done!\n------");
}

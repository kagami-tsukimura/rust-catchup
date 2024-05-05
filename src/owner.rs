pub fn run() {
    println!("owner.rs is start!");

    // 所有権：s1
    let s1 = String::from("hello");
    // 所有権：s1 → s2
    let s2 = s1;
    // s1に所有権がない（s2にmove）ためアクセスできずにエラー
    // String型は`Copy` trait を実装していないため
    // println!("{}, {}", s1, s2);
    println!("{}", s2);

    // 整数、浮動小数点、配列型は`Copy` trait を実装している
    // moveが起きずにスタック内で値がコピー→エラーにならない
    let x1 = 5;
    let x2 = x1;
    println!("{} {}", x1, x2);
    println!("Stack address of x1, x2 is: {:p}, {:p}", &x1, &x2);

    println!("owner.rs is done!\n------");
}

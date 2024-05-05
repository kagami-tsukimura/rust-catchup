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
    let x1 = 5;
    let x2 = x1;
    println!("{} {}", x1, x2);
    // moveが起きずにスタック内で値がコピー→エラーにならない
    println!("Stack address of x1, x2 is: {:p}, {:p}", &x1, &x2);

    // 文字列スライス: 値を参照
    let sl1 = "literal";
    let sl2 = sl1;
    // moveが起きずにスタック内で値がコピー→エラーにならない
    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1, sl2 is: {:p}, {:p}", &sl1, &sl2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    // deep copyすると値がコピーされる→所有権がmoveせずエラーを防げる
    println!("{} {}", s3, s4);
    println!("Stack address of s3, s4 is: {:p}, {:p}", &s3, &s4);
    println!(
        "Heap address of s3, s4 is: {:?}, {:?}\n",
        &s3.as_ptr(),
        &s4.as_ptr()
    );

    println!("owner.rs is done!\n------");
}

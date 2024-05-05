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

    // 関数の呼び出し
    let s5 = String::from("hello");
    print_ownership(&s5);
    // Heapをそのまま引き継ぐ
    take_ownership(s5);
    // take_ownership()の中で所有権がmove→s5に所有権がなくなりエラー
    // Heapが引き継がれたため
    // println!("{}", s5);

    // s6, s, s7自体はスタック内の別メモリに生成
    // 実データは所有権としてmove
    let s6 = String::from("hello");
    print_ownership(&s6);
    // 関数の返却時にs6→s7に所有権がmove
    let s7 = take_giveback_ownership(s6);
    // print_ownership(&s6);
    print_ownership(&s7);

    let s8 = String::from("hello");
    // calculate_length: &Stringで参照型のため、s8, s9ともに使用可能
    let s9 = calculate_length(&s8);
    println!("{}", s8);
    println!("{}", s9);

    let mut s10 = String::from("hello");
    change(&mut s10);
    println!("{}", s10);

    // immutableなref: 複数作成可
    // mutableなref: 複数作成不可
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // // immutableとmutableなref: 共存不可
    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // //  データの整合性を保つため共存不可
    // let r2 = &mut s10;
    // println!("{} {} {}", s10, r1, r2);

    let mut s11 = String::from("hello");
    let r3 = &mut s11;
    // // mutableな領域である場合、所有権者であってもエラー
    // println!("{}", s11); // エラー
    // println!("{}", r3);

    // r3の有効期限が先に切れるためok
    println!("{}", r3);
    println!("{}", s11); // ok

    let mut s12 = String::from("hello");
    let r4 = &s12;
    let r5 = &s12;
    println!("{} {}", r4, r5);
    // ref
    let r6 = &mut s12;
    *r6 = String::from("hello_updated!");
    println!("{}", r6);
    println!("{}", s12);

    println!("owner.rs is done!\n------");
}

// 変数の引数、関数の戻り値とした場合も所有権のmoveが発生
// 関数を抜ける際に戻り値(s)が解放→Heap内の実データを破棄
fn take_ownership(s: String) {
    // &: 所有権の借用
    print_ownership(&s);
}

fn print_ownership(s: &String) {
    println!("\nprint_ownership: {}------", s);
    println!("{}", s);
    println!("Stack address of s is: {:p}", &s);
    println!("Heap address of s is: {:?}", &s.as_ptr());
    println!("Length of s is: {}", s.len());
    println!("Capacity of s is: {}", s.capacity());
    println!("------print_ownership: {}\n", s);
}

fn take_giveback_ownership(s: String) -> String {
    // Rustではreturn文はない
    // 最後にセミコロンのない値をreturnとみなして返す
    s
}

fn calculate_length(s: &String) -> usize {
    // s: &Stringで参照型のため、所有権がmoveしない
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}

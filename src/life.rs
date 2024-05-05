pub fn run() {
    let x = 5;
    // xのアドレス取得
    let r = &x;
    println!("{}", x);
    println!("{}", r);

    // x: 関数をdropする際に削除
    // y: 使用後に削除
}

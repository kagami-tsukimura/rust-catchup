pub fn run() {
    let x = 5;
    // xのアドレス取得
    let r = &x;
    println!("{}", r);
    println!("{}", x);

    // x: 関数をdropする際に削除
    // y: 使用後に削除

    // // ダングリングポインタ
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);
}

pub fn run() {
    // lifecycle
    let x = 5;
    // xのアドレス取得
    let r = &x;
    println!("{}", r);
    println!("{}", x);
    // x: 関数を抜ける際にdrop
    // y: 使用後に削除

    // // ダングリングポインタ
    // let r;
    // {
    //     let x = 5;
    //     r = &x;

    //     // x: スコープを抜ける際にdrop
    // }
    // println!("r: {}", r);
}

pub fn run() {
    println!("ref_borrow.rs is start!");

    // 参照(reference): &によるアドレス取得→所有権＋プリミティブ型両方
    // - String, Vec, Box, Rc, Arc
    // 借用(borrowing): 所有権を移動させず、参照権だけを貸す
    // プリミティブ型は所有権がない→借用の概念なし
    // - 整数型: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize
    // - 浮動小数点型: f32, f64
    // - 論理型: bool
    // - 文字型: char
    // - バイト型: u8

    // Rustのメモリ安全性
    // 所有権システム→二重解放エラーを防ぐ
    // RAII→メモリ解放忘れを防ぐ（変数初期化時にメモリ確保→スコープ抜けると解放）
    // ライフタイム→ダングリングポインタを防ぐ（コンパイラが指摘）

    // ジェネリックライフタイムアノテーション
    let st1 = String::from("xz");
    let st2 = String::from("y");
    println!("st1 = {}, st2 = {}", st1, st2);
    println!("longest = {}", get_longest(&st1, &st2));

    let st3 = String::from("z");
    // 型：逆算して推論（get_longest()の返り値から推論）
    // let res2;
    // ライフタイムが異なるケース
    {
        let st4 = String::from("scoped");
        let res = get_longest(&st3, &st4);
        println!("{}", res);
        // // ダングリングポインタ: st4はスコープ抜けると解放
        // res2 = get_longest(&st3, &st4);
        // printlnをスコープ内で行えば可
        // println!("{}", res2);
    }
    // st4の実体: スコープを抜けてdrop
    // res2のポインタが指し示す先がないためエラー
    // println!("{}", res2);

    println!("{}", dummy3());
}

// x, yどちらのライフタイムを返り値に適用するかわからないためエラー
// <'a>: ジェネリックライフタイムアノテーション
// →引数(x, y)のうち、短い方のライフタイムを適用する
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn dummy1<'a>() -> &'a str {
//     let s = String::from("dummy");
//     // s: 実体が関数を抜ける際にdrop
//     // →referenceを返すとダングリングポインタが発生するためエラー
//     &s
// }

// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     // s: 実体が関数を抜ける際にdrop
//     // →referenceを返すとダングリングポインタが発生するためエラー
//     &x
// }

fn dummy3() -> String {
    let s = String::from("dummy");
    // s: 実体を返すためdropしない
    s
}

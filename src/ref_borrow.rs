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
    // RAII→メモリ解放忘れを防ぐ
    // ダングリングポインタ→ライフタイムの制限を防ぐ
}

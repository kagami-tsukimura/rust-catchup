pub fn run() {
    println!("hand.rs is start!");

    // Option型: 値が存在しないものをエラーハンドリング
    // Result型: エラーが発生する可能性がある値をエラーハンドリング
    // →Error時のパニック（処理の停止）をせずにハンドリング可能

    println!("hand.rs is done!\n------");
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

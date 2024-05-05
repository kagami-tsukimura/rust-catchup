pub fn run() {
    println!("hand.rs is start!");

    // Option型: 値が存在しないものをエラーハンドリング
    // Result型: エラーが発生する可能性がある値をエラーハンドリング
    // →Error時のパニック（処理の停止）をせずにハンドリング可能

    let res1 = division_option(5.0, 2.0);
    match res1 {
        // 正常系: if y != 0.0
        Some(x) => println!("Result: {:.3}", x),
        // 異常系: if y == 0.0
        None => println!("Error: Division by zero."),
    }

    let res1 = division_option(5.0, 0.0);
    match res1 {
        // 正常系: if y != 0.0
        Some(x) => println!("Result: {:.3}", x),
        // 異常系: if y == 0.0
        None => println!("Error: Division by zero."),
    }

    println!("hand.rs is done!\n------");
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

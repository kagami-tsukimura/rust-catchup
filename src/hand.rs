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

    let res2 = division_result(5.0, 0.0);
    match res2 {
        // 正常系: if y != 0.0
        Ok(x) => println!("Result: {:.3}", x),
        // 異常系: if y == 0.0
        Err(e) => println!("{}", e),
    }

    let a = [0, 1, 2];
    let res3 = sum(&a);
    match res3 {
        Some(x) => println!("Sum: {}", x),
        None => println!("Error: Array length is less than 3."),
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

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Division by zero."))
    } else {
        Ok(x / y)
    }
}

fn sum(a: &[i32]) -> Option<i32> {
    // "?"でエラーキャッチ→returnで、Option列挙型のNoneを返す
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    // エラーキャッチなし→returnで、Someを返す
    Some(a0 + a1 + a2)
}

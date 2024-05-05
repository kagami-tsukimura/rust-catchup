struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 1, 2引数の面積を比較し、1が大きければtrueを返す
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn double_value(x: i32) -> i32 {
    x * 2
}
fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// testを実行するために必要
// cargo testを実行したときのみコンパイル
#[cfg(test)]
mod tests {
    // 上記の関数より1階層下に入っている
    // 関数にアクセスできるようにする
    use super::*;
    // テストコードを実行するために必要
    #[test]
    fn test_a_is_larger() {
        let a = Rectangle {
            width: 5,
            height: 5,
        };
        let b = Rectangle {
            width: 3,
            height: 3,
        };
        // a > bならテストが通る
        assert!(a.compare_area(&b));
    }
}

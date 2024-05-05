// 構造体: Typeのようなもの
// 型をジェネリクスにすることも可能
// 複数同じジェネリクスを指定する場合、変数の型も合わせる必要がある
// →x: f64, y: f64など（x: f64, y: i32はエラー）
struct Point<T> {
    x: T,
    y: T,
}

pub fn run() {
    let numbers = vec![34, 50, 25, 100, 65];

    println!("largest: {}", get_largest(numbers));

    // ダブルクォート: 文字列リテラル
    // シングルクォート: char
    let chars = vec!['a', 'b', 'c', 'd', 'f', 'e'];
    println!("largest: {}", get_largest(chars));

    // // トレード境界に属さない型はエラー
    // let strings: Vec<String> = vec![
    //     String::from("hello"),
    //     String::from("rust"),
    //     String::from("is"),
    //     String::from("awesome"),
    // ];
    // println!("largest: {}", get_largest(strings))

    let p1 = Point { x: 1, y: 2 };
    // 構造体に合っていない型はエラー
    // 構造体をジェネリクスにして回避
    let p2 = Point { x: 3.0, y: 2.0 };
    println!("{} {}", p1.x, p2.y);
}

// T: 何らかの型
// T: PartialOrd + Copy: トレード境界
// トレード境界を指定することで、任意の型に対応できる
// →比較できない型もあるため、比較演算子を使う場合はなければエラー
fn get_largest<T: PartialOrd + Copy>(items: Vec<T>) -> T {
    let mut largest = items[0];
    for item in items {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn print_generics(s: &String) {
//     println!("\nprint_generics: {}------", s);
//     println!("{}", s);
//     println!("Stack address of s is: {:p}", &s);
//     println!("Heap address of s is: {:?}", &s.as_ptr());
//     println!("Length of s is: {}", s.len());
//     println!("Capacity of s is: {}", s.capacity());
//     println!("------print_generics: {}\n", s);
// }

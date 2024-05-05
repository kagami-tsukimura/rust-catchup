pub fn run() {
    let numbers = vec![34, 50, 25, 100, 65];

    println!("largest: {}", get_largest(numbers));

    // ダブルクォート: 文字列リテラル
    // シングルクォート: char
    let chars = vec!['a', 'b', 'c', 'd', 'f', 'e'];
    println!("largest: {}", get_largest(chars));
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
pub fn run() {
    let numbers = vec![34, 50, 25, 100, 65];

    println!("largest: {}", get_largest_i32(numbers));
}

fn get_largest_i32(items: Vec<i32>) -> i32 {
    // Vector<i32>内の最大値を取得
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

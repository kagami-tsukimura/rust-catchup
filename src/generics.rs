pub fn run() {
    let numbers = vec![34, 50, 25, 100, 65];
    let mut largest = numbers[0];
    println!("largest: {}", largest);

    largest = get_largest(numbers);
    println!("largest: {}", largest);
}

fn get_largest(numbers: Vec<i32>) -> i32 {
    // Vector内の最大値を取得
    let mut largest = 0;
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn print_generics(s: &String) {
    println!("\nprint_generics: {}------", s);
    println!("{}", s);
    println!("Stack address of s is: {:p}", &s);
    println!("Heap address of s is: {:?}", &s.as_ptr());
    println!("Length of s is: {}", s.len());
    println!("Capacity of s is: {}", s.capacity());
    println!("------print_generics: {}\n", s);
}

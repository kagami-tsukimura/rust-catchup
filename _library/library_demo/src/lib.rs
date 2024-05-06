mod generator;

pub fn print_random_number() {
    // gen_ran()はu8型
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}

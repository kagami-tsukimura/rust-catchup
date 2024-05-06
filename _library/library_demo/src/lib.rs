mod generator;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn print_random_number() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

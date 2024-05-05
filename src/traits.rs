// trait: 複数の型に共通の機能を提供
trait Fruits {
    fn price(&self) -> u32;
    fn eat(&self);
}
struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        100
    }
    fn eat(&self) {
        println!("eat apple");
    }
}

pub fn run() {
    println!("trait.rs is start!");
}

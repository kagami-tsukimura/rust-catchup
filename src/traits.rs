// trait: 複数の型に共通の機能を提供
trait Fruits {
    fn price(&self) -> u32;
    fn eat(&self) -> &str;
}
struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        100
    }
    fn eat(&self) -> &str {
        "apple"
    }
}

struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        40
    }
    fn eat(&self) -> &str {
        "banana"
    }
}

pub fn run() {
    println!("trait.rs is start!");

    let apple = Apple {};
    let banana = Banana {};

    // 参照渡し: get_price()後に所有権を持ってget_eat()で呼び出しできるように、"&"をつける
    get_price(&apple);
    get_eat(&apple);
    get_price(&banana);
    get_eat(&banana);
}

fn get_price<T: Fruits>(fruits: &T) {
    // &T: 関数終了時に所有権を解放しないように参照渡し
    println!("price is: ${}", fruits.price())
}

fn get_eat<T: Fruits>(fruits: &T) {
    println!("eat is: {}", fruits.eat())
}

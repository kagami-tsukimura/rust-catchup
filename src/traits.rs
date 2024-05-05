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

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // format: {}をString型に変換
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
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
    get_price(&apple);
    get_price(&banana);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("summarize: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Rustacean"),
        content: String::from("The Pittsburgh Penguins once again are the best team in the NHL."),
    };
    println!("article: {}", article.summarize());
}

fn get_price<T: Fruits>(fruits: &T) {
    // &T: 関数終了時に所有権を解放しないように参照渡し
    println!("price is: ${}", fruits.price())
}

fn get_eat<T: Fruits>(fruits: &T) {
    // &T: 関数終了時に所有権を解放しないように参照渡し
    println!("eat is: {}", fruits.eat())
}

fn notify(item: &impl Summary) {
    println!("notify: {}", item.summarize());
}

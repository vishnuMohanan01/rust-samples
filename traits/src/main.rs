use std::fmt::{Display, Debug};

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        self.username.clone()
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.content, self.username)
    }
}

struct NewsArticle {
    author: String,
    headline: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

// This is an easy way
// fn notify(item: &impl Summary) {
//     println!("Breaking news: {}", item.summarize());
// }

// Called trait bound
fn notify<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

fn multi_notify<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news: {}", item1.summarize());
    println!("Breaking news: {}", item2.summarize());
}

fn some_function_1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
fn some_function_2<T, U>(t: &T, u: &U)
    where T: Display + Clone,
          U: Clone + Debug
{

}

fn returns_summarizable_1(t: &impl Summary) -> &impl Summary{
    t
}

fn returns_summarizable_2<T: Summary>(t: &T) -> &T{
    t
}

mod conditional_trait;

// ====
// blanket implementation

// some trait
trait ToString {}

// implementing ToString on types that has implemented Display
impl<T: Display> ToString for T {

}
//===


fn main() {
    let tweet = Tweet {
        username: "themv37".to_string(),
        content: "Hello, rust twitter".to_string(),
        reply: false,
        retweet: false
    };
    let article = NewsArticle {
        headline: "This headline".to_string(),
        author: "Vishnu".to_string(),
        content: "This is the content".to_string()
    };

    println!("tweet summary: {}", tweet.summarize());
    println!("article summary: {}", article.summarize());

    notify(&tweet);
    notify(&article);

    // multi_notify(&tweet, &article); error - article is of different type
    multi_notify(&tweet, &tweet); // works - both same type


}

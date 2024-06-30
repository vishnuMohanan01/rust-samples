use std::fmt::Display;

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_with_an_announcement<'a, T> (s1: &'a str, s2: &'a str, ann: T) -> &'a str
where
    T: Display
{
    println!("This is the announcement: {}", ann);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn s1_r_same_s2_diff_lifetime() {
    let s1 = String::from("xyz");
    let r: &str;

    {
        let s2 = String::from("abcd");
        r = longest(s1.as_str(), s2.as_str());
    }
    // println!("The longest str is: {}", r); -- throws error due to insufficient lifetime of s2
}

fn s1_diff_s2_r_same_lifetime() {
    let s1 = String::from("abcd");

    {
        let s2 = String::from("xyz");
        let r = longest(s1.as_str(), s2.as_str());
        println!("The longest str is: {}", r);
    }
}

fn s1_s2_r_same_lifetime() {
    let s1 = String::from("abcd");
    let s2 = String::from("xyz");

    let r = longest_with_an_announcement(s1.as_str(), s2.as_str(), "Woohooo!");
    println!("The longest str is: {}", r);
}

fn main() {
    s1_s2_r_same_lifetime();
    s1_diff_s2_r_same_lifetime();
    s1_r_same_s2_diff_lifetime();
}

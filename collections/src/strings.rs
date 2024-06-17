use unicode_segmentation::UnicodeSegmentation;

pub fn main () {
  let s1 = String::new();
  let s2 = "hello world";
  let s3 = s2.to_string();
  let s4 = "hey, world!".to_string();
  let s5 = String::from("hey, world! --using String::from");

  let mut s6 = String::from("hello, ");
  s6.push_str("string");
  s6.push('!');
  println!("{}", s6);

  let s7 = "Hey, ".to_string();
  let s8 = "there --using + operator".to_string();
  let s9 = s7 + &s8;
  println!("{}", s9);

  let s10 = "Hey, ".to_string();
  let s11 = "there --using format! macro".to_string();
  let s12 = format!("{} {}", s10, s11);
  println!("{}", s12);

  println!("\n\n---Start utf-8 experiments in Hindi---\n");
  let s13 = "नमस्ते".to_string();
  for b in s13.bytes() {
    println!("{}", b);
  }

  for s in s13.chars() {
    println!("{}", s);
  }

  for s in s13.chars() {
    println!("{}", s);
  }

  for g in s13.graphemes(true) {
    println!("{}", g);
  }
  println!("\n---End utf-8 experiments in Hindi---\n\n");
}

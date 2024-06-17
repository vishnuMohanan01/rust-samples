use std::collections::HashMap;

pub fn main() {
  let blue = "blue".to_string();
  let yellow = "yellow".to_string();

  let mut scores: HashMap<String, i32> = HashMap::new();

  scores.insert(blue, 10);
  scores.insert(yellow, 20);

  let blue = "blue".to_string();
  println!("Score of blue is: {:?}", scores.get(&blue).unwrap());

  for (key, value) in &scores {
    println!("Key: {} -- Value: {}", key, value);
  }

  scores.entry("yellow".to_string()).or_insert(40);
  scores.entry("yellow".to_string()).or_insert(60);

  for (k, v) in &scores {
    println!("{}: {}", k, v);
  }

  let text = "hello world wonderful world".to_string();
  let mut word_count: HashMap<String, u8> = HashMap::new();
  for word in text.split_whitespace() {
    let count = word_count.entry(word.to_string()).or_insert(0);
    *count += 1;
  }

  println!("{:?}", word_count);

  let value = word_count.entry("hello".to_string());
  *value = 7;
}

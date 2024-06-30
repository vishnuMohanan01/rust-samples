use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmd_display(&self) {
    if self.x > self.y {
      println!("the largest is: {}", self.x);
    } else {
      println!("the largest is: {}", self.y);
    }
  }
}

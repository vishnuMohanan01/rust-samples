struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}

pub fn add_two(num: i32) -> i32 {
  num + 2
}

pub fn greet(name: &str) -> String {
  format!("Hello, {}", name)
}

struct Guess {
  value: i32
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 {
      panic!("The value should be greater than or equal to 1")
    } else if value > 100 {
      panic!("The value should be less than or equal to 100")
    }

    Guess {
      value
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      width: 8,
      height: 5
    };

    let smaller = Rectangle {
      width: 4,
      height: 2
    };

    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cant_hold_larger() {
    let larger = Rectangle {
      width: 8,
      height: 5
    };

    let smaller = Rectangle {
      width: 4,
      height: 2
    };

    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  fn it_adds_two() {
    assert_eq!(4, add_two(2));
  }

  #[test]
  fn it_do_not_add_three() {
    assert_ne!(5, add_two(2));
  }

  #[test]
  fn it_contains_name() {
    let name = "Vishnu";

    let result = greet(name);

    assert!(result.contains(name), "Greeting didn't contain name. Value was `{}`", result);
  }

  #[test]
  #[should_panic]
  fn it_panics_if_guess_is_less_than_1() {
    Guess::new(0);
  }

  #[test]
  #[should_panic(expected = "The value should be less than or equal")] // expected can be any substring
  fn it_panics_if_guess_is_greater_than_100() {
    Guess::new(101);
  }
}

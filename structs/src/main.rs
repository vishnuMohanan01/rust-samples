#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  
  fn can_hold(&self, another_rectangle: &Rectangle) -> bool {
    self.width > another_rectangle.width && self.height > another_rectangle.height
  }
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {width: size, height: size}
  }
}

fn main() {
  let rect_1 = Rectangle {width: 15, height: 20};
  let rect_2 = Rectangle {width: 2,height: 3};
  let rect_3 = Rectangle {width: 50, height: 60};
  
  let square_1 = Rectangle::square(100);
  
  println!("The rectangle is: {:#?}", rect_1);
  println!("The area is: {} pixels", rect_1.area());
  println!("Rect 1 can hold 2 ?: {}", rect_1.can_hold(&rect_2)); // true
  println!("Rect 1 can hold 3?: {}", rect_1.can_hold(&rect_3)); // false
  
  println!("The sqaure is: {:#?}", square_1);
  println!("The area is: {} pixels", square_1.area());
  println!("Sqaure 1 can hold Rect 1 ?: {}", square_1.can_hold(&rect_1)); // true
  println!("Square 1 can hold Rect 2 ?: {}", square_1.can_hold(&rect_2)); // true
  println!("Sqaure 1 can hold Rect 3 ?: {}", square_1.can_hold(&rect_3)); // true
}

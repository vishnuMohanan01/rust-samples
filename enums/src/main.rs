#[derive(Debug)]
enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6,
}

impl IpAddrKind {
  fn some_fn() {
    println!("Say something!");
  }
}


fn main() {
  let ip_v4 = IpAddrKind::V4(127, 0, 0, 1);
  
  println!("The ip is {:?}", ip_v4);
  println!("The association func gets called below");
  IpAddrKind::some_fn();
  
  // Using Option enum
  let x = 5;
  let y: Option<i32> = None;
  let sum = x + y.unwrap_or(0);
  
  println!("Sum is: {}", sum);
  
  let x  = 10;
  println!("Just adding one to x: {}", plus_one(Some(x)).unwrap());
  println!("Just adding one to None: {:?}", plus_one(None));
  
  // Some with a specific value
  let some_value = Some(3);
  match some_value {
    Some(3) => println!("The value is three!"),
    _ => println!("Not three")
  }
  // Using if-let for the same
  if let Some(3) = some_value {
    println!("the value is three! -- using if-let")
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

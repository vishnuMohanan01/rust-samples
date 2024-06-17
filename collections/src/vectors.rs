pub fn main() {
  let a = [1, 2, 3];

  println!("{:?}", a);

  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);

  let v2 = vec![1, 2, 3];

  let third = &v2[2];
  println!("the third element is: {}", third);

  match v2.get(20) {
    Some(third) => println!("the third element is: {} -- using get", third),
    None => println!("There is no element at the given index!")
  }

  println!("Printing in loop");

  let mut v3 = vec![1, 2, 3, 4, 5, 6];
  for i in &mut v3 {
    // Adding to a &i32 does not make sense.
    // So, using dereference operator to mutate actual value.
    *i += 50;
  }

  for i in &v3 {
    println!("{}", i);
  }

  enum SpreadsheetCell {
    Int(i32),
    Text(String)
  }

  let v4 = vec![SpreadsheetCell::Int(1), SpreadsheetCell::Text(String::from("hello"))];

  for i in &v4 {
    match i {
      SpreadsheetCell::Int(integer_value) => println!("The integer value is: {}", integer_value),
      _ => println!("Not an integer cell value")
    }
  }
}

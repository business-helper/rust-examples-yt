fn main() {
  func1();

  func2();
}

fn func1() {
  let a = [1,2, 3];
  let mut v:Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
}

fn func2() {
  let v = vec![1,2,3,4,5];

  let third = &v[3];
  // let third = &v[20]; // index out of bounds error.

  println!("The third element is {}", third);

  // process exceptions for the index-out-of-bounds error.
  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is not third element"),
  }
}
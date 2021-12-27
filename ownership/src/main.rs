fn main() {
  let x = 5;
  let y = x; // Copy
  
  let s1 = String::from("hello");
  // let s2 = s1; // Move(not shallow copy)
  let s2 = s1.clone(); // Move(not shallow copy)
  println!("{}, world!", s1);

  func_ownership();

  let x = 5;
  makes_copy(x);
  println!("{}", x);

  println!("[Gives Onwership]");
  let s3 = gives_ownership();
  println!("s3={}", s3);

  println!("[Take & Give Back]");
  let s4_1 = String::from("Hello S4-1");
  let s4 = takes_and_gives_back(s4_1);
  println!("s1 = {}, s4 = {}", s1, s4);

  // let len = calculate_length(s1);
  // println!("The length of '{} is {}.", s1, len);
}

fn func_ownership() {
  let s= String::from("hello S3");
  takes_ownership(s);
  // println!("{}", s); // this will throw an error due to the ownership

  fn takes_ownership(some_string: String) {
    println!("{}", some_string);
  }
}

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn gives_ownership() -> String {
  let some_string = String::from("hello");
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

fn calculate_length(s: String) -> usize {
  let length = s.len(); // len() returns the length of a String.
  // (s, length)
  length
}

use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
  func1();

  func2();

  func3();

  func4();

  func5();

  func6();

  func7();

  func8();

  func9();
}

fn func1() {
  let a = [1,2, 3];
  let mut v:Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
}

fn func2() {
  let mut v = vec![1,2,3,4,5];

  let third = &v[3];
  // let third = &v[20]; // index out of bounds error.

  // this line causes syntax error as v is already borrwed as a immutable reference.
  // v.push(6);

  println!("The third element is {}", third);

  // process exceptions for the index-out-of-bounds error.
  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is not third element"),
  }
}

// iterating vector
fn func3() {
  let mut v = vec![1,2,3,4,5];

  for i in &mut v {
    *i += 50;
  }

  for i in &v {
    println!("{}", i);
  }
}

// array of enum
fn func4() {
  println!("----- func4 -----");

  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];

  match &row[1] {
    SpreadsheetCell::Int(i) => println!("{}", i),
    _ => println!("Not a integer")
  };
}

// various strng definition
fn func5() {
  println!("----- func5 -----");

  // strings are stroed as a collection of UTF-8 encoded bytes.
  let s1 = String::new();
  let s2 = "initial contents";
  let s3 = s2.to_string();
  let s4 = String::from("initial contents");

  let mut s = String::from("foo");
  s.push_str(" bar");
  s.push('!');

  println!("mutated string is {}", s);
}

fn func6() {
  println!("----- func6 -----");

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3: String = s1 + &s2;

  println!("merged string: {}", s3);

  // string is the bytes array in fundamental.
  for b in "العددي".bytes() {
    println!("{}", b);
  }

  // we also can iterate the string as a scalar values //العددية
  for c in "العددي".chars() {
    println!("{}", c);
  }

  // iterate a string as a grapheme clusters.
  for g in "العددي".graphemes(true) {
    println!("{}", g);
  }
}

// HashMap
fn func7() {
  println!("----- func7 -----");
  let blue = String::from("Blue");
  let yellow = String::from("Yellow");

  let mut scores = HashMap::new();

  scores.insert(blue, 10);
  scores.insert(yellow, 50);

  // blue ownership -> hashmap.
  // println!("{}", blue);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);
  
  // println!("Blue team score is {:?}", score);

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }
}

fn func8() {
  println!("----- func8 -----");
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 20);

  scores.entry(String::from("Yellow")).or_insert(30);
  scores.entry(String::from("Yellow")).or_insert(40);

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }
}

fn func9() {
  println!("----- func9 -----");

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}

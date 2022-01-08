use std::fs::File;
use std::io::ErrorKind;

fn main() {
  func5();

  return;
  func4();
  func3();
  func2();
  func1();
}

// get unwrap type
fn func5() {
  println!("------ func 5 ------");

  let f = File::open("hello.txt").unwrap();
  // let f = File::open("hello.txt").expect("Failed to open hello.txt");

  println!("File opened successfully!");
}

// error with file manipulations
fn func4() {
  println!("------ func 4 ------");

  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    // Err(error) => panic!("Problem opening the file: {:?}", error),
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    }
  };

  // rewrite using if clause.
  let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
}

// error with enum
fn func3() {
  println!("------ func 3 ------");

  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
}

// --------- simple example
fn func1() {
  panic!("crash and burn");
}

// ---------- nested calls
fn func2() {
  println!("------ func 2 ------");
  a();
}

fn a() {
  b();
}

fn b() {
  c(22);
}

fn c(num: i32) {
  if num == 22 {
    panic!("Don't pass in 22!");
  }
}

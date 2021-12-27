enum IpAddrKind {
  V4(String),
  V6(String),
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn some_function() {
    println!("Some function message")
  }
}

fn main() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  let localhost = IpAddr {
    kind: IpAddrKind::V4(String::from("127.0.0.1")),
    address: String::from("127.0.0.1")
  };

  func1();

  func2();

  func3();

  func4();
}

fn func4() {
  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(i) => Some(i + 1)
    }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
}

fn func3() {
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
  }

  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
    }
  }

  println!("A dime is {} cents.", value_in_cents(Coin::Dime));
}

fn func2() {
  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  let sum = x + y.unwrap_or(0);
}

fn func1() {
  // enum Options<T> {
  //   Some(T),
  //   None,
  // }
  
  let some_number = Some(5);
  let some_string = Some("a string");
  let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) {

}

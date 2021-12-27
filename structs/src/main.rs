struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area0(width1, height1)
  );

  let rect = (30, 50);
  println!(
    "The area of the rectangle is {} square pixels.",
    area1(rect)
  );

  let rectangle = Rectangle {
    width: 20,
    height: 30,
  };
  println!("rect: {:?}", rectangle);
  println!(
    "The area of the rectangle is {} square pixels.",
    area2(&rectangle)
  );

  let rectangle2 = Rectangle {
    width: 50,
    height: 60,
  };
  println!("rect: {:#?}", rectangle2);
  println!(
    "The area of the rectangle is {} square pixels.",
    rectangle2.area()
  );

  let rect1 = Rectangle {
    width: 20,
    height: 40
  };

  let rect2 = Rectangle {
    width: 40,
    height: 50
  };

  println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
  println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
}

fn area2(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}

fn area1(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area0(width: u32, height: u32) -> u32 {
  width * height
}

fn main0() {
  let user1 = User {
    email: String::from("business@helper.com"),
    username: String::from("business"),
    active: true,
    sign_in_count: 1
  };

  let name = user1.username;
  // user1.username = String::from("bus");

  let user2 = build_user(String::from("second@test.com"), String::from("second"));
  let user3 = User{
    email: String::from("third@test.com"),
    username: String::from("third"),
    ..user2
  };
}

fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    sign_in_count: 1,
    active: true,
  }
}
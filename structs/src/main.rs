struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1, height1)
  );
}

fn area(width: u32, height: u32) -> u32 {
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
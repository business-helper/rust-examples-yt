fn main() {
    func3();
    return;
    func2();
    func1();
    // return;
}

struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn x(&self) -> &T {
    &self.x
  }

  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

impl<T, f64> Point<T, f64> {
  fn y(&self) -> &f64 {
    &self.y
  }
}

enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}

fn func3() {
  let p1 = Point { x: 5, y: 10 };
  let p2 = Point { x: 5.0, y: 10.0 };
  let p3 = Point { x: 5, y: 10.0 };
  let p4 = Point { x: "Hello", y: 'c' };
  let p5 = p3.mixup(p4);

  println!("p5.x = {}, p5.y = {}", p5.x, p5.y);

}

// apply generic type
fn func2() {
    println!("------- func 1 --------");
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = get_largest(char_list);

    println!("The largest number is {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// get the maximum value from a vector.
fn func1() {
    println!("------- func 1 --------");

    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);
}

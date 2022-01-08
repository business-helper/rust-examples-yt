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

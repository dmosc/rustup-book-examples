#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// Specific impls for concrete types such as f32.
// This function will only be available for Point<f32>.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> Point<T> {
    fn clone<T1>(self, other: Point<T1>) -> Point<T1> {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}

fn main() {
    let list = vec![1, 2, 9, 4, 2, 5];
    println!("{}", largest(&list));

    let point = Point { x: 1, y: 2 };
    println!("point.x(): {}, point.y(): {}", point.x(), point.y());

    let f_point: Point<f32> = Point { x: 1.1, y: 2.2 };
    println!(
        "point<f32>.distance_from_origin(): {}",
        f_point.distance_from_origin()
    );

    let new_point = point.clone(f_point);
    // After .clone(...) call, `point` and `f_point` are not valid
    // since their references moved into the function call.
    println!("{:?}", new_point);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    
}

fn largest<T>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn main(){
    let number_list = vec![1,2,3,4,5,6];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let both_integer = Point{x: 5, y: 10};
    let both_float = Point{x: 1.0, y: 4.0};
    let integer_float = DifferentPoint{x: 1, y: 1.5};
}

//generic struct defs
struct Point<T> {
    x: T,
    y: T,
}

//method will only be available for T = f64
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct DifferentPoint<X, Y> {
    x: X,
    y: Y,
}

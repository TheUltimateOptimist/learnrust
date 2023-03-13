// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x*2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//     println!("The value of x is: {x}");
// }

fn main() {
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;
    let _c: char = 'c';
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let (x, y, z) = tup;
    let b = [3; 3];//same as b = [3, 3, 3] first number is value and second number is length
    let c = a[0];
    let d = b[2];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("The value of y is: {y}");
    let z = five();
    println!("The value of x is: {z}");
}

fn five() -> i32{
    5
}

fn mainTwo(){
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {number}")
}

fn main3(){
    let condition = true;
    let number = if condition {5} else {"6"};//both arms need to have same return type
    println!("The value of number is: {number}");
}

fn main4(){
    loop {//infinite loop
        println!("again!");
        //can break or skip manually using break or continue
    }
}

//basic loop
fn main5(){
    let mut counter = 0;
    let result = loop {//loop will return 20
        counter+=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

//labeled loops
fn main6() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

//while loop
fn main7(){
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number-=1;
    }
    println!("LIFTOFF!!!")
}

//while loop through collection
fn main8(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index+=1;
    }
}

//for in loop
fn main9(){
    let a = [10, 20, 30, 40, 60];
    for element in a {
        println!("the value is: {element}");
    }
}

//countdown with for loop
fn main10(){
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

//explicit/implicit reference/dereference
fn main11(){
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
}







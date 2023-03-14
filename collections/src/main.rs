fn main() {
    let v: Vec<i32> = Vec::new();
    let w = vec![1, 2, 3];
    let mut z = Vec::new();
    z.push(5);
    z.push(6);
    z.push(7);

    let vec = vec![1, 2, 3, 4, 5];
    let third: &i32 = &vec[2];
    println!("{:?}", vec);
    println!("{}", third);
    let third: Option<&i32> = vec.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    for n_ref in &vec {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{}", n_plus_one);
    }

    let mut mutable = vec![100, 32, 57];
    for n_ref in &mut mutable {
        *n_ref += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

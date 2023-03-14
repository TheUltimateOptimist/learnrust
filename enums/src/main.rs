// enum IpAddr {
//     V4(String), 
//     V6(String),
// }

// fn route(ip_kind: IpAddrKind) {

// }

// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
//     println!("Hello, world!");
// }


//new version
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main(){
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
// }

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //method body would be defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("Luckky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


enum IpAddrKind {
    V4(String),
    V6(String),
}
impl Message {
    fn call(&self) {}
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// fn exlusive_test(x: u32) {
//     match x {
//         x % 2 == 0 => {println!("I'm divisible by 2");}
//         x % 3 == 0 => {println!("I'm divisible by 3");}
//         x % 6 == 0 => {println!("I'm divisible by 2 and 3");}
//         }
// }

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);
    let m = Message::Write(String::from("Hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {}

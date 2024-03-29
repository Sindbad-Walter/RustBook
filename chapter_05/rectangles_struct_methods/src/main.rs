#[derive(Debug)]
struct User {
    username: String,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 25,
        height: 55,
    };
    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     rect1.area()
    // );

    // println!(
    //     "{}",
    //     rect1.can_hold(&rect2)
    // );

    let sq = Rectangle::square(3);
    let user1 = build_user(String::from("Sindbad"), true);
    println!("{:?}", &user1);
    let user2 = User {
        username: user1.username.clone(),
        active: false,
    };
    println!("{:?}", &user1);
    println!("{:?}", &user2);
}

fn build_user(username: String, active: bool) -> User {
    User { username, active }
}

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

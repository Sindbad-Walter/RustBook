fn main() {

  //  let x = 5;

  //let x = x + 1;

  //{
  //    let x = x * 2;
  //    println!("The value of x in the inner scope is: {}", x)
  //}

  //println!("The value of x is: {}", x);


   let y ={
       let x = 3;
       x + 1
   };

  //  println!("The value of x is {}", y);
  //  let my_number_is = 12;
  //  let my_other_number_is = tryme(my_number_is);
  //  println!("The value of my other number is: {}", my_other_number_is);
  //  let condition: bool;
  //  if 5 > 4 {
  //    condition = true;
  //  } else {
  //    condition = false;
  //  }

  //  let win = if { }

  //  let mut count = 0;
  //  'counting_up: loop {
  //    println!("count = {}", count);
  //    let mut remaining = 10;

  //    loop{
  //      println!("remaining = {}", remaining);
  //      if remaining == 9 {
  //        break;
  //      }
  //      if count == 2{
  //        break 'counting_up;
  //      }
  //      remaining -= 1;
  //    }
  //    count += 1;
  //  }

  //  println!("End count = {}", count);

  //Reversing a string
  // let my_string = String::from("HELLO");
  // let mut my_reverse_string = String::from("");
  //  for my_char in my_string.chars().rev() {
  //     my_reverse_string.push(my_char);
  //  }
  //  println!("{}", my_reverse_string);
  // let reference_to_nothing = dangle();
let s = String::from("Hello my name is sindbad");
// println!("{}", first_word(&s));


let a = [1,2,3,4,5];
let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
struct User{
  active: bool,
  username: String,
  email: String,
  sign_in_count: usize
}

let user1 = User {
  email: String::from("sindbad.walter@gmail.com"),
  username: String::from("SindbadWalter"),
  active: true,
  sign_in_count: 1,
};

let user2 = User {
  active: user1.active,
  username: user1.username,
  email: String::from("another@example.com"),
  sign_in_count: user1.sign_in_count
};

let user3 = User {
  email: String::from("third@example.com"),
  ..user1
};

}


//Testing how writing a function other than main looks like
//   fn tryme(number: u32) -> u32 {
//   number*5
// }


// fn dangle() -> String {
//   let s = String::from("hello");
//   s
// }

fn first_word(s: &String) -> &str{
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate(){
    if item ==b' '{
      return &s[..i];
    }
  }
  &s[..]
}

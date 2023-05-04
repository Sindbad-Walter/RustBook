fn main() {
    // Uncomment to test celsius_to_fahrenheit function
    // let my_fahr_temp = celsius_to_fahrenheit(12.0);
    // println!("The conversion is {}", my_fahr_temp);

    let my_nth_fib = generate_nth_fib_number(15);
    println!("{}", my_nth_fib);

    let mut _spaces = 5;
    let _spaces = "   ";
    // spaces += 1; //Throws a compile error as we have shadowed 'spaces' to a str, and try to apply an operation not
    //not supported on
    let quotient = 13 / 5;
    println!("{}", quotient);
    use std::io;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    if index > a.len() {
        panic!("Index is too large")
    };
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn celsius_to_fahrenheit(temperature_in_celsius: f32) -> f32 {
    let fahr_temp = temperature_in_celsius * 1.8 + 32.0;
    fahr_temp
}

fn generate_nth_fib_number(wanted_fib_number: u64) -> u64 {
    if wanted_fib_number == 1 {
        0
    } else if wanted_fib_number == 2 {
        1
    } else {
        let mut prev = 0;
        let mut curr = 1;
        let mut count = 1;
        while count != wanted_fib_number {
            let next_fib = curr + prev;
            // next_fib = curr + prev;
            prev = curr;
            curr = next_fib;
            count += 1
        }
        curr
    }
}

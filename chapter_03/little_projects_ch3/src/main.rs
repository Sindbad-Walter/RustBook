fn main() {
    // Uncomment to test celsius_to_fahrenheit function
    // let my_fahr_temp = celsius_to_fahrenheit(12.0);
    // println!("The conversion is {}", my_fahr_temp);

    let my_nth_fib = generate_nth_fib_number(15);
    println!("{}", my_nth_fib);
}

fn celsius_to_fahrenheit(temperature_in_celsius: f32) -> f32 {
    let fahr_temp = temperature_in_celsius*1.8 + 32.0;
    fahr_temp
}

fn generate_nth_fib_number(wanted_fib_number: u64) -> u64 {
    if wanted_fib_number == 1{
        0
    } else if wanted_fib_number == 2{
        1
    } else {
        let mut prev = 0;
        let mut curr = 1;
        let mut count = 1;
        while count != wanted_fib_number{
            let next_fib: u64;
            next_fib = curr + prev;
            prev = curr;
            curr = next_fib;
            count += 1
        }
        curr
    }
}

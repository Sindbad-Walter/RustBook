struct ImportanteExcerpt<'a> {
    part: &'a str,
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for number in list {
        if *number > largest {
            largest = *number;
        }
    }
    largest
}

fn gen_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['a', 'b', 'c', 'd'];

    let largest_number = gen_largest(&number_list);
    let largest_char = gen_largest(&char_list);

    println!("The largest number is {}", largest_number);
    println!("The largest char is {}", largest_char);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 2.4, y: 34 };

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("The longest string is {result}");
    }
    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportanteExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

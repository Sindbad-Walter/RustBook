fn main() {
    use std::collections::HashMap;
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];

    println!("{}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // let mut vect = [1, 2, 3, 4, 5, 6];
    // for i in &mut vect {
    //     *i += 50;
    // }

    // for i in &vect {
    //     println!("{}", i);
    // }
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let __row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(2.4),
        SpreadSheetCell::Text(String::from("Hello")),
    ];

    let s1 = String::from("Hello,");
    let s2 = " World!".to_string();

    let hello = "Здравствуйте";

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world what a wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

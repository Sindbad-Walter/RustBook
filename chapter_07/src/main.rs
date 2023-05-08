use crate::garden::vegetables;

pub mod garden;

fn main() {
    let plant = vegetables::Asparagus {};
    println!("I'm growing {:?}!", plant);
    let plant2 = vegetables::zuchini_maker(12, vegetables::Looks::Good);
    println!("{:?}", plant2);
}

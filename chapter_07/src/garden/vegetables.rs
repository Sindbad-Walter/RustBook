#[derive(Debug)]
pub struct Asparagus {}
#[derive(Debug)]
pub struct Zuchini {
    size: u32,
    looks: Looks,
}

#[derive(Debug)]
pub enum Looks {
    Bad,
    Average,
    Good,
}

pub fn zuchini_maker(size: u32, looks: Looks) -> Zuchini {
    Zuchini { size, looks }
}

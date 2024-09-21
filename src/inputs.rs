#[derive(Debug)]
pub enum Inputs {
    Number(i64),
    Add,
    Subtract,
    Multiply,
}

impl Inputs {
    fn enum_index(&self) -> u8 {
        use Inputs::*;
        match *self {
            Number(_) => 0,
            Add => 1,
            Subtract => 1,
            Multiply => 2,
        }
    }
}

impl PartialOrd for Inputs {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.enum_index().cmp(&other.enum_index()))
    }
}

impl PartialEq for Inputs {
    fn eq(&self, other: &Self) -> bool {
        self.enum_index() == other.enum_index()
    }
}

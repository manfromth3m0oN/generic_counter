use crate::counter::Countable;

#[derive(Clone)]
pub struct Pear {
    name: String,
}

impl Pear {
    pub fn new(name: String) -> Pear {
        return Pear { name };
    }
}

impl Countable for Pear {
    fn get_count(&self) -> usize {
        return 1;
    }
}

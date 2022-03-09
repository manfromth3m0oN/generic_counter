use crate::counter::Countable;

#[derive(Clone)]
pub struct Tupaware<T> {
    pub colour: String,
    contents: Vec<T>,
}

impl<T> Tupaware<T> {
    pub fn new(colour: &str) -> Tupaware<T> {
        return Tupaware {
            colour: String::from(colour),
            contents: Vec::new(),
        };
    }

    pub fn add_item(&mut self, item: T) {
        self.contents.push(item);
    }
}

impl<T> Countable for Tupaware<T> {
    fn get_count(&self) -> usize {
        self.contents.len()
    }
}

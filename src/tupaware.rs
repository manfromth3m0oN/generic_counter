use crate::counter::Countable;

#[derive(Clone)]
pub struct Tupaware<T: Countable + ?Sized> {
    pub colour: String,
    contents: Vec<Box<T>>,
}

impl<T: Countable> Tupaware<T> {
    pub fn new(colour: &str) -> Tupaware<T> {
        return Tupaware {
            colour: String::from(colour),
            contents: Vec::new(),
        };
    }

    pub fn add_item(&mut self, item: T) {
        self.contents.push(Box::new(item));
    }
}

impl<T: Countable + ?Sized> Countable for Tupaware<T> {
    fn get_count(&self) -> usize {
        let total: usize = 0;
        for item in &self.contents {
            total += item.get_count();
        }
        total
    }
}

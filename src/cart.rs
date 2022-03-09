use crate::{counter::Countable, tupaware::Tupaware};

pub struct Cart<T: Countable + ?Sized> {
    pub id: String,
    contents: Vec<Box<Tupaware<T>>>,
}

impl<T: Countable + ?Sized> Cart<T> {
    pub fn new(id: &str) -> Cart<T> {
        return Cart {
            id: String::from(id),
            contents: Vec::new(),
        };
    }

    pub fn add_tupaware(&mut self, tupaware: Tupaware<T>) {
        self.contents.push(Box::new(tupaware))
    }
}

impl<T: Countable + ?Sized> Countable for Cart<T> {
    fn get_count(&self) -> usize {
        let mut total: usize = 0;
        for tupaware in &self.contents {
            total += tupaware.get_count();
        }
        total
    }
}

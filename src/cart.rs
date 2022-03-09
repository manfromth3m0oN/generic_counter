use crate::{counter::Countable, tupaware::Tupaware};

pub struct Cart<T> {
    pub id: String,
    contents: Vec<Tupaware<T>>,
}

impl<T> Cart<T> {
    pub fn new(id: &str) -> Cart<T> {
        return Cart {
            id: String::from(id),
            contents: Vec::new(),
        };
    }

    pub fn add_tupaware(&mut self, tupaware: Tupaware<T>) {
        self.contents.push(tupaware)
    }
}

impl<T> Countable for Cart<T> {
    fn get_count(&self) -> usize {
        let mut total: usize = 0;
        for tupaware in &self.contents {
            total += tupaware.get_count();
        }
        total
    }
}

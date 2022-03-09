pub mod apple {
    pub struct Apple {
        name: String,
    }

    impl Apple {
        pub fn new(name: String) -> Apple {
            return Apple { name: name };
        }
    }
}

pub mod counter {
    pub struct Counter<T> {
        pub items: Vec<T>,
    }

    impl<T> Counter<T> {
        pub fn add_apples(&mut self, item: T) {
            self.items.push(item)
        }

        pub fn get_count(&self) -> usize {
            return self.items.len();
        }
    }
}

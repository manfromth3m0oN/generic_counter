use crate::counter::Countable;

#[derive(Clone)]
pub struct Apple {
    name: String,
}

impl Apple {
    pub fn new(name: String) -> Apple {
        return Apple { name: name };
    }
}

impl Countable for Apple {
    fn get_count(&self) -> usize {
        return 1;
    }
}

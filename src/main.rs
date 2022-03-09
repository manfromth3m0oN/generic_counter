use counter::{apple::Apple, counter::Counter};

// Create an empty Apple class
// Create a Counter class
// Can add Apples to the class
// Can getCount() which returns the total count so far
fn main() {
    let mut counter = Counter::<Apple> { items: Vec::new() };
    for i in 0..10 {
        counter.add_apples(Apple::new(format!("{}", i)))
    }

    println!("{}", counter.get_count());
}

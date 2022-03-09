mod apple;
mod cart;
mod counter;
mod tupaware;
use crate::apple::Apple;
use crate::cart::Cart;
use crate::counter::Countable;
use crate::tupaware::Tupaware;

// Create an empty Apple class
// Create a Counter class
// Can add Apples to the class
// Can getCount() which returns the total count so far
fn main() {
    let purple_tupware = gen_filled_tupaware("purple");
    let green_tupaware = gen_filled_tupaware("green");

    let mut first_cart: Cart<Apple> = Cart::new("first");
    first_cart.add_tupaware(purple_tupware.clone());
    first_cart.add_tupaware(green_tupaware.clone());

    println!(
        "{} Tupaware Count: {}",
        purple_tupware.colour,
        purple_tupware.get_count()
    );
    println!(
        "{} Tupaware Count: {}",
        green_tupaware.colour,
        green_tupaware.get_count()
    );

    println!("{} Cart count: {}", first_cart.id, first_cart.get_count())
}

fn gen_filled_tupaware(colour: &str) -> Tupaware<Apple> {
    let mut tupaware = Tupaware::new(colour);
    for i in 0..10 {
        let apple = Apple::new(format!("{}", i));
        tupaware.add_item(apple);
    }
    tupaware
}

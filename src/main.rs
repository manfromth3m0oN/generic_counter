mod apple;
mod cart;
mod counter;
mod pear;
mod tupaware;
use crate::apple::Apple;
use crate::cart::Cart;
use crate::counter::Countable;
use crate::pear::Pear;
use crate::tupaware::Tupaware;

// Create an empty Apple class
// Create a Counter class
// Can add Apples to the class
// Can getCount() which returns the total count so far
fn main() {
    let purple_tupware = gen_filled_apple_tupaware("purple");
    let green_tupaware = gen_filled_apple_tupaware("green");
    let blue_tupaware = gen_filled_pear_tupware("blue");
    let pink_tupaware = gen_filled_pear_tupware("pink");

    let mut first_cart: Cart<dyn Countable> = Cart::new("first");
    first_cart.add_tupaware(purple_tupware.clone());
    first_cart.add_tupaware(green_tupaware.clone());
    first_cart.add_tupaware(blue_tupaware.clone());
    first_cart.add_tupaware(pink_tupaware.clone());

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

fn gen_filled_apple_tupaware(colour: &str) -> Tupaware<dyn Countable> {
    let mut tupaware = Tupaware::new(colour);
    for i in 0..10 {
        let apple = Apple::new(format!("{}", i));
        tupaware.add_item(apple);
    }
    tupaware
}

fn gen_filled_pear_tupware(colour: &str) -> Tupaware<dyn Countable> {
    let mut tupaware = Tupaware::new(colour);
    for i in 0..10 {
        let pear = Pear::new(format!("{}", i));
        tupaware.add_item(pear);
    }
    tupaware
}

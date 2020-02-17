use typed_phy::{Quantity, units::{Newton, Watt}};

fn main() {
    let _: Quantity<i32, Newton> = Quantity::<i32, Watt>::new(0);
}

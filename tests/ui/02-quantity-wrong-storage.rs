use typed_phy::{Quantity, units::MetrePerSecond};

fn main() {
    let _: Quantity<u32, _> = Quantity::<i32, MetrePerSecond>::new(0);
}

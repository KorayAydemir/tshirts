use tshirts::{self, Inventory, ShirtColor};
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red);

    let giveaway1 = store.giveaway(user_pref1.clone());

    println!(
        "User with preference {:?} gets {:?}",
        &user_pref1, giveaway1
    );
}

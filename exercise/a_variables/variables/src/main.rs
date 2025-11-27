const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    // let unused_variable = 1;
    // READY_AMOUNT = 1;

    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("{} missiles left", missiles - ready);
}

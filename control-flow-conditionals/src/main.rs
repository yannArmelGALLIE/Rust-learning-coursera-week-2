fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not Proceeding");
    }

    let masse = 190;
    if masse > 180 {
        println!("Tall");
    } else if masse > 170 {
        println!("Average");
    } else {
        println!("Short");
    }
}
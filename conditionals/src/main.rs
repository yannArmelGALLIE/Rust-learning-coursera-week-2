fn main() {
    let maybe_number = Some(42);

    if let Some(number) = maybe_number {
        println!("Le nombre est : {}", number);
    } else {
        println!("Ce n'est pas un nombre");
    }
}
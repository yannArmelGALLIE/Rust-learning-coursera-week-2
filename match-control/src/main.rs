use std::io;

fn main() {
    let mut salutation = String::new();
    println!("Entrez une salutation : ");
    io::stdin().read_line(&mut salutation).expect("Erreur dans la lecture de l'entrée");

    match salutation.trim() {
        "Bonjour" => println!("Enchante de vous rencontrer"),
        "Au revoir" => println!("Desole de vous voir partir"),
        _ => println!("Je ne peux pas trouver la salutation, au revoir"),
    }
}
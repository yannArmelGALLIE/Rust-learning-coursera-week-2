use std::io;

fn main() {
    let mut input = String::new();

    while input.trim() != "stop" {
        input.clear();
        println!("Entrez un mot (Entrez 'stop' pour sortir) : ");
        io::stdin().read_line(&mut input).expect("Erreur de lecture de l'entree");
        println!("Vous avez entre : {}", input);
    }
    println!("Au revoir");
}
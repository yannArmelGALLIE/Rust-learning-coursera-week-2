use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

fn main() {
    let file = File::open("file.txt");

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("Fichier introuvable : {}", error)
            }
            _ => {
                panic!("Erreur dans l'ouverture du fichier : {}", error)
            }
        }
    }
}
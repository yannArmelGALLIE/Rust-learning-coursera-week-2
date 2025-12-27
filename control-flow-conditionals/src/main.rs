fn main() {
    let age = 19;
    let sexe = true;

    if age > 18 {
        if sexe {
            println!("Homme adulte");
        } else {
            println!("Femme adulte");
        }
    } else if age > 14 {
        if sexe {
            println!("Un adolescent");
        } else {
            println!("Une adolescente");
        }
    } else {
        if sexe {
            println!("Un enfant");
        } else {
            println!("Une enfant");
        }
    }
}
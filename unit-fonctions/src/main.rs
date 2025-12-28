fn process_numbers(numbers: &[i32]) {
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }

    println!("La somme des nombres est : {}", sum);

    if sum % 2 == 0 {
        println!("La somme est paire");
    } else {
        println!("La somme est impaire");
    }
}

fn main() {
    let numbers = [1, 2, 8];
    process_numbers(&numbers);
}
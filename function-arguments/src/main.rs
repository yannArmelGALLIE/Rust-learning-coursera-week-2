fn sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}

fn main() {
    let numbers = [1, 2, 3];
    let sum = sum(&numbers);
    println!("La somme est : {}", sum);
}
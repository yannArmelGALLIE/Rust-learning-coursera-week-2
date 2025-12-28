fn loop_and_panic(vec: Vec<i32>) {
    for num in vec {
        if num < 0 {
            panic!("Nombre negatif trouve");
        }
        println!("Le nombre est : {}", num);
    }
}

fn main() {
    let vec = vec![1, 2, 3, 4, -5];
    loop_and_panic(vec);
}
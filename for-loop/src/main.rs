fn main() {
    for x in 1..10 {
        println!("x : {}", x);
    }

    // for x in (1..=10).rev() {
    //     println!("x : {}", x);
    // }

    let nombres = vec![1, 2, 3, 4, 5];
    for n in nombres {
        println!("n : {}", n);
    }
}
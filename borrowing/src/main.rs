fn own_integer(i: i32) {
   let _ =  i + 1;
}

fn own_string(text: String) {
    println!("{}", text);
}

fn own_vec(mut vec: Vec<i32>) {
    vec.push(10);
    println!("{:?}", vec);
}

fn main() {
    let nombre = 10;
    let text = String::from("Hello, world");
    let mut vec = vec![1, 2, 3];

    own_integer(nombre);
    println!("{}", nombre);

    own_string(text);

    own_vec(vec);
}
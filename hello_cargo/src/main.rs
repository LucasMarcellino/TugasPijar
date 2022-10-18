// fn main() {
//     let a = "Hello";
//     let b = "world";
//     let output = format!("{} {}", a, b);

//     print!("{}", output);
// }

// 1. Concat String
// Buatlah sebuah function yang menerima 2 buah parameter berupa tipe data String,
// param1 = "I love"
// param2 = "Rust"

// param1 = "I love"
// param2 = "Rust"
// output = "I love Rust"


fn main() {
    let param1: String = "I Love".to_owned();
    let param2: String = "Rust".to_owned();

    let output = format!("{} {}", param1, param2);
    println!("{}", output);
}


// 2. The Remainder
// Kalian bisa menggunakan operator % (remainder) untuk soal berikut
// Buatlah sebuah function yang menerima 2 buah parameter berupa integer,

// param1 = 10
// param2 = 3

fn main() {
        
        let param1 = 10;
        let param2 = 3;

        let _remainder = param1 % param2;
    

        let output = format!("{}", _remainder);
        println!("the result is : {}", output);
}

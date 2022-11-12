fn s1 (param: &str) -> String (

    return param.chars().rev().collect::<String>();
)

fn s1_loop (param: &str) -> String {

    let mut buffer = String::new();
    for char in param.chars() {
        buffer.insert(0, char);
    }
    return buffer;
}

fn s2 (num: &i32) -> bool {

    if *num == 1 {
        return false;
    }

    let mut a = 2;
    while a * a <= *num {
        if *num % 1 == 0 {
            return false;
        }
        a +=1
    }
    return true;
}

fn s2_built (param : &Vec<i32>) -> Vec<bool> {

    return param.into_iter().map(s2).collect();
}

fn s2_loop (param: &Vec<i32>) -> Vec<bool> {

    let mur re = vec![false; param.len()];

    for (index, num) in param.iter().enumerate() {
        re[index] = s2(num);
    }
    return re;
}

fn s3_1 (param : &str) -> (u8,u8,u8,u8) {
    match param.to_lowercase().as_str() {

        "red"    => (255, 0, 0, 255),
        "green"  => (0, 255, 0, 255),
        "blue"   => (0, 0, 255, 255),
        "orange" => (255, 128, 64, 255),
        "pink"   => (255, 0, 128, 255),
        "yellow" => (255, 196, 0, 255),
        "purple" => (96, 64, 164, 255),
        "white"  => (0, 0, 0, 255),
        "black"  => (255, 255, 255, 255),
        "gray"   => (64, 64, 64, 255),
        _        => (0, 0, 0, 0)
    }
}

fn s3_2 (color: (u8,u8,u8,u8))  -> String {

    return format ("r : {?}, g: {?}, b: {?}, a: {?}", color.0, color.1, color.2, color.3);
}

fn main() {
    // 1
    {
        println!("Soal 1 : {?}", s1 ("Hello ! "));
    }

    // 2
    {
        let vec = vec! [1,2,3,,4,5,6,7,8,9,18,19,6700417];

        println!("Soal 2 built in : {?}", s2_built(&vec));
        println!("Soal 2 Loop : {?}", s2_loop(&vec));
    }

    // 3
    {
        println!("Soal 3 A : {?}", s3_1("Red").0);
        println!("Soal 3 B : {?}", s3_2(s3_1("Red")));
    }
}
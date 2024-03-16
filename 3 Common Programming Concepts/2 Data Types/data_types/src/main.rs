fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0;
    let y: f32 = 3.0;

    // operations

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    // bool

    let t = true;
    let f: bool = false;

    // char

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    // tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}"); // will == 6.4
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array

    let months = ["January", "February", "March", "April", "May", "June", "July",
                              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // Same as let b = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];

}

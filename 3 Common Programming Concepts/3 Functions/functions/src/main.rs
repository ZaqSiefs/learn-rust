fn main() {
    function(5);
    print_labeled_measurements(5, 'h');
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    let x = five();
    println!("The value of x is: {x}");
    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
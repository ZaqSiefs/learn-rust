fn main() {
    let mut number = 3;
    compare(number);
    number = 7;
    compare(number);
    number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
    number = 6;
    divisible(number);

    let condition = true;
    number = if condition { 5 } else { 6 };
    println!("number is {number}");

    counter();
    counting_up();
    liftoff();
    for_collection();
    liftoff_idio();
}

fn compare(x: i32) {
    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn divisible(x: i32) {
    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        print!("number is not divisible by 4, 3, or 2");
    }
}

fn counter() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn counting_up() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn liftoff() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_collection() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {element}");
    }
}

fn liftoff_idio() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
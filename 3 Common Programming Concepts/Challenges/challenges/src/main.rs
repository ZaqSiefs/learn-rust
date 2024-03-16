use std::io;

fn main() {
    temp();
    fibonacci();
    days_of_christmas();
}

fn temp() {
    loop {
        println!("Enter a temperature in Farenheit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let temp: f64 = match input
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid Input");
                    continue;
                }
            };
        let c_temp: f64 = f_to_c(temp);
        println!("{temp} farenheit in celcius = {c_temp}");
        break;
    }

    loop {
        println!("Enter a temperature in Celcius");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let temp: f64 = match input
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid Input");
                    continue;
                }
            };
            let f_temp: f64 = c_to_f(temp);
            println!("{temp} celcius in farenheit = {f_temp}");
            return;
    }
}

fn c_to_f (temp: f64) -> f64 {
    ((temp * (9.0/5.0) * 100.0).round() / 100.0) + 32.0
}

fn f_to_c (temp: f64) -> f64 {
    ((temp - 32.0) * (5.0/9.0) * 100.0).round() / 100.0
}

fn fibonacci() {
    loop {
        println!("Enter an index from 0 - 186");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let index: u8 = match input
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid Input");
                    continue;
                }
            };
        if 186 < index {
            println!("Invalid Input");
            continue;
        }
        let fibonacci_number: u128 = get_fibonacci(index);
        println!("Fibonacci number = {fibonacci_number}");
        return;
    }
}

fn get_fibonacci(index: u8) -> u128 {
    if index == 0 || index == 1 {return index as u128};
    
    let mut number: u128 = 1;
    let mut prev_number: u128 = 0;
    for _i in 2..=index {
        let temp = number;
        number += prev_number;
        prev_number = temp;
    }
    return number;
}

fn days_of_christmas() {
    for i in 1..=12 {
        let suffix: &str = match i {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!("On the {i}{suffix} day of Christmas my true love sent to me");
        for j in (1..=i).rev() {
            match j {
                1 => println!("a partridge in a pear tree!"),
                2 => println!("two turtle doves and"),
                3 => println!("three french hens"),
                4 => println!("four calling birds"),
                5 => println!("five gold rings"),
                6 => println!("six geese a-laying"),
                7 => println!("seven swans a-swimming"),
                8 => println!("eight maids a-milking"),
                9 => println!("nine ladies dancing"),
                10 => println!("ten lords a-leaping"),
                11 => println!("eleven pipers piping"),
                12 => println!("twelve drummers drumming"),
                _ => println!(""),
            }
        }
    }
}
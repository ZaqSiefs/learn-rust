fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    // x = 6; // Cannot do because x is immutable

    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds = {THREE_HOURS_IN_SECONDS} seconds");
    // Shadowing
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();  // Will use either depending on type needed


}

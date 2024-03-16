fn main() {
    // size of string (literal) must be known by compile time
    let _s1 = "hello";

    // size of string (String) can be unknown and mutable at compile time
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("{s2}");

    let s3 = s2;
    let s4 = s3.clone();
    println!("s1 = {}, s2 = {}", s3, s4);

    // Function ownership

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let _s5 = gives_ownership();         // gives_ownership moves its return
                                    // value into s5

    let s6 = String::from("hello");     // s6 comes into scope

    let _s7 = takes_and_gives_back(s6);  

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}
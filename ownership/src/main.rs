fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    // the following is not a copy but a move which invalidates s
    // uncomment the two lines and run to see Rust shout at you!!
    // let s1 = s;
    // println!("{}", s);

    // You can use the clone method to perform deep copy and avoid moving
    let s1 = s.clone();
    println!("{}", s);
    println!("{}", s1);

    // s1's value moves into the function...
    take_ownership(s1);
    let x = 5;
    makes_copy(x); // Ok since x is a primitive type => a copy is made, not a move.
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}, {}", s1, s3);
    let (s4, len) = calculate_length(s3);
    println!("the length of '{}' is {}.", s4, len);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// Returning values can also transfer ownership.
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn ownership_examples() {
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

fn reference_and_borrowing_examples() {
    let s1 = String::from("hello");
    let len = calculate_length_from_ref(&s1); // reference borrowing. value of s1 is not dropped
    println!("The length from reference of '{}' is {}.", s1, len);
    // Rust prevents more than one mutable reference. to work around it, create a new scope
    let mut s = String::from("Yannick");
    {
        let r1 = &mut s;
        r1.push_str(" from the inner scope");
        println!("{} **", r1);
        // safely use r1 here
    } // r1 goes out of scope here
    let r2 = &mut s;
    println!("{}", r2);
    // let dangling_reference = dangle();
    let valid_string = no_dangle();
    // println!("{}", dangling_reference);
    println!("{}", valid_string);
}

fn main() {
    ownership_examples();
    reference_and_borrowing_examples();
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

fn calculate_length_from_ref(s: &String) -> usize {
    s.len()
}

// fn dangle() -> &String {
fn no_dangle() -> String {
    let s = String::from("hello");
    // &s
    s
} // Here s goes out of scope and is dropped. Danger!!

fn create_and_update_a_vector() {
    let v = vec![1, 2, 3];
    let mut vector = Vec::new();
    vector.push("Go to Market");
    vector.push("Buy some fish");
    vector.push("Cook some food");
    println!("{:?}", v);
    println!("{:?}", vector);
}

fn reading_elements_from_vector() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // see how access behaves when index out of bound
    // let does_not_exist = &v[100]; // this panics
    let does_not_exist = v.get(100); // this fails gracefully
    match does_not_exist {
        Some(value) => println!("the value is {}", value),
        None => println!("No value found at this index"),
    }
}

fn iterating_over_vector_values() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    println!("Before: {:?}", v);
    for i in &mut v {
        *i += 50;
    }
    println!("After: {:?}", v);
}
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    create_and_update_a_vector();
    // reading element of Vectors
    reading_elements_from_vector();
    // iterating over the values in a vector
    iterating_over_vector_values();
    // using enum to store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.13),
    ];
    println!("{:?}", row);
}

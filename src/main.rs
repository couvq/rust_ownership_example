fn main() {
    // s is allocated on the heap and mutable, so we can store value of unknown size
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{s}"); // s is valid. Stores known quantity on stack with pointer to where values are stored on the hep
    let s2 = s; // s is moved to s2 and s is no longer valid
    println!("s2 is {s2}");
    // println!("s2 is {s2} and s is {s}") won't compile since s is no longer valid and we are trying to use it here
    let s3 = give_ownership();
    println!("s3 is {s3} and now has ownership from the give_ownership function");
    // next lesson:
    // if you want to use a value without transferring ownership, you can use references
}

fn give_ownership() -> String {
    let s = String::from("hello");
    s
}

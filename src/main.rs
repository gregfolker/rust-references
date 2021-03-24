// Project: references
// Author: Greg Folker

fn main() {
	println!("Hello, World!");

    // References allow you to refer to some value
    // without taking ownership of it
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    // We are allowed to use `s1` here because it was
    // passed by reference above, the scope of `s1`
    // did not change since the ownership did not change
    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("Hello");

    // This is how pass by reference works by default in C
    // In Rust however, `&mut` must be used to let the compiler
    // know that the contents of the memory are able to be changed
    // within the new scope we are passing `s` into
    change(&mut s);

    println!("{}", s);

    // The one big restriction of mutable references is that
    // you can only have one mutable reference to a particular
    // piece of data within a particular scope
    // For example:
    let mut s = String::from("Hello");

    let r1 = &mut s;

    // This is a compiler error "cannot borrow `s` as mutable more than
    // once at a time"
    // let r2 = &mut s;

    println!("{}", r1);

    // The benefit of having this restriction on mutable variables is that
    // Rust can prevent race conditions at compile time
    // Curly brackets can be used to create a new scope, however, which
    // allows multiple mutable references, but not simultaneous ones
    let mut s = String::from("Hello");

    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here and the `drop()` method is called

    // We can make a new reference without a problem now because there is no other
    // reference to `s` within the current scope since it's memory was freed back to the heap
    let r2 = &mut s;
    println!("{}", r2);

    // A similar rule exists for combining mutable and immutable references
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    // Line 68 is a compiler error because we are trying to borrow
    // `s` as mutable when it was borrowed as immutable above by
    // `r1` and `r2`, which could result in a race condition later on
    // if `r3` modifies the value of `s`
    // let r3 = &mut s;

    println!("r1={} and r2={}", r1, r2);

    // We can, however, create a mutable reference to `s` after
    // we have already used (or, read from) `r1` and `r2`
    // The scopes of `r1` and `r2` end after they have last been used (Line 70)
    let r3 = &mut s;
    println!("r3={}", r3);

    // The Rust compiler also protects you against accidentally creating
    // a dangling pointer, which is a pointer that references a location
    // in memory that may have been given to someone else. If you have
    // a reference to some data, the compiler will ensure that the data
    // will not go out of scope before the reference to the data does
    // let reference_to_nothing = dangle();

    let reference_to_something = no_dangle();

    println!("reference_to_something={}", reference_to_something);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // `s` goes out of scope here, but because the function
  // did not have ownership of it, it cannot call the `drop()`
  // method on it

// This is a compiler error because the parameter some_string
// is not explicitely labeled as mutable with `&mut`
// fn change(some_string: &String) {
//    some_string.push_str(", world");
// }

// To pass by reference with the intention of modifying
// the value, the passed in variable and parameter must be
// mutable
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("Hello");

     // The problem with this return on Line 111 is that `s` goes
     // out of scope as soon as this function ends. Its memory goes away entirely,
     // so we are returning a reference to nothing
     // &s
// }

// To avoid the dangling pointer problem, the `String` needs to be returned directly,
// which ensures that "not nothing" will be returned to the caller
fn no_dangle() -> String {
    let s = String::from("Hello");

    // Ownership is moved back to the caller and nothing is deallocated
    s
}

fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;    // z is an immutable reference to x
    *y += 1;       // Modifying x through y is allowed
    println!("x = {}", x); // Output: x = 6
    //println!("z = {}", *z); // Error! z is immutable, and cannot be dereferenced to modify x
}
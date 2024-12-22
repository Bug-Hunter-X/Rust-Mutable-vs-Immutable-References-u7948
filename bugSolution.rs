fn main() {
    let mut x = 5;
    { // Creating a new scope to limit the lifetime of the mutable reference
        let y = &mut x; 
        *y += 1;  
    }
    let z = &x; // Now we can safely use the immutable reference z
    println!("x = {}, z = {}", x, *z); // Output: x = 6, z = 6
} 
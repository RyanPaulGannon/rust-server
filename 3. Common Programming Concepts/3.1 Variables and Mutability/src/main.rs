fn main() {
    // Use 'mut' to allow mutability, e.g. let mut x = 5 will allow the value to be changed
    // Contants aren't allowed to be used with mut
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

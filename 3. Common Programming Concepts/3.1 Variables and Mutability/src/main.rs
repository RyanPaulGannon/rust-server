fn main() {

    test_function();

    // Example of a statement - these return a vlue
    let x = 5;

    /*
        Multi line comment test, below is example of an expression which returns a value 
    */
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn test_function() {
    println!("A function inside a function")
}

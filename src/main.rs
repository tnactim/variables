fn main() {
    // notes about data type annotation

    let x = 5;
    println!( "The value of x is: {}", x );
    let x = 6; // shadow of x, otherwise immutable unless 'let mut'
    println!( "The value of x is: {}", x );

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // explicit type annotation

    // char
    let c = 'z';
    let z = 'Z';
    let ok_hand = '👌';
    // how do I use a codepoint instead of the emoji itself?
    //let one_hundred = U+1F4AF;

    // Compound Types
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays
    // on the stack instead of the heap
    // fixed number of elements, unlike a vector
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // array element indexing
    let first = a[0];
    let second = a[1];
    let index = 10; // out of bounds

    let element = a[index - 6];
    println!("The value of element is: {}", element);
}

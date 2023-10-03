fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5;
    // first variable being shadowed by the second one.
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

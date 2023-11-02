fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // let x = 5;
    // first variable being shadowed by the second one.
    // let x = x + 1;
    //
    // {
    //     let x = x * 2;
    //     println!("The value of x in inner scope is: {x}");
    // }
    //
    // println!("The value of x is: {x}");
    let x: (i32, f64, u8) = (500, 4.1, 1);
    let five_hundred = x.0;

    let a = [3;5];

    println!("The value of x is: {five_hundred}");

}

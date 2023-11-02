// fn main() {
//
//     another_function(5, 'h');
// }
//
// fn another_function(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }
//
// fn main() {
//   let t = ([1;2], [3;4]);
//   let (a, b) = t;
//   println!("{:?}", b[0]); 
// }
fn main() {

  let t = ([1; 2], [3; 4]);

  let (a, b) = t;

  println!("{}", a[0] + t.1[0]); 

}

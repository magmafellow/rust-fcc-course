// --= first task --solved
// fn main() {
//   let x: i32 = 5;
//   let y: i32;

//   assert_eq!(x, 5);
//   println("Success");
// }

// --= second task --solved
// fn main() {
//   let mut x: i32 = 1;
//   x += 2; // x = x + 2
//   // 3
  
//   assert_eq!(x, 3);
//   println!("Success");

// }

// --= third task --solved

// [ a scope is the range within the program for which the item is valid ]
// fn main() {
//   let x: i32 = 10;
//   let y: i32 =5;

//   {
//     println!("The value of x is {} and value of y is {}", x, y);
//   }

//   println!("The value of x is {} and value of y is {}", x, y);
// }

// --= fourth task --solved
// fn main() {
//   define_x();
// }

// fn define_x() {
//   let x: &str = "hello";

//   println!("{}, world", x);
// }

// --= fifth task --solved
// fn main() {
//   let x: i32 = 5;
//   {
//     let x = 12;
//     assert_eq!(x, 5);
//   }

//   assert_eq!(x, 12);

//   let x = 42;
//   println!("{}", x);
// }

// --= sixth task --solved
// fn main() {
//   let mut x: i32 = 1;
//   x = 7;
//   // shaddowing and re-binding
//   x += 3

//   let y = 4;
//   // shaddowing
//   let y = "I can also be bound to text!";

//   println!("Success")
// }

// --= seventh task --solved
// #[allow(unused_variables)]
// fn main() {
//     let x = 1;
// }

// --= eighth task --solved
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
//     println!("Success");
// }

// --= nineth task --solved
fn main() {
    let (x, y);

    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}

// All topic is DONE
// Great

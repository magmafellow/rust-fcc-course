// A unit type is a type that holds no value and costs 0 bytes

fn first() {
  let _v: () = ();

  let v: (i32, i32) = (2, 3);
  assert_eq!(_v, implicitly_ret_unit());

  println!("Well");
}

fn implicitly_ret_unit() {
  println!("I will return a ()");
}

// bad practice
fn explicitly_ret_unit() -> () {
  println!("I will return a ()");
}

// ---------------------------------------
// What is the size of the unit type?
use std::mem::size_of_val;
fn second(){
  let unit: () = ();
  assert!(size_of_val(&unit) == 0);
  print!("Well");
}

// ---------------------------------------

// there is something interesting here :)
fn first() {
  let x: u32 = 5u32;

  let y: u32 = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    // this expression will be assigned to `y`
    x_cube + x_squared + x
  };

  let z = {
    // the semicolon suppresses this expression and `()` is assigned to `z`
    2 * x; // due to semicolon at the end it does not return a number but a unit
  };

  println!("x is {:?}", x);
  println!("y is {:?}", y);
  println!("z is {:?}", z);  
}

// -----------------------------------------

fn second_v1() {
  let v: u8 = {
    let mut x = 1;
    x += 2;
    x
  };

  assert_eq!(v, 3);

  println!("good");
}
fn second_v2() {
  let v = {
    let mut x = 1;
    x += 2;
  };

  assert_eq!(v, ());

  println!("good");
}

// -----------------------------------------

fn third() {
  let v = {
    let x = 3;
    x
  };

  assert!(v == 3);
  
  print!("good");
}

// -----------------------------------------

fn four() {
  let s = sum(1, 2);
  assert_eq!(s, 3);

  println!("good");
}
fn sum(x: i32, y: i32) -> i32 {
  x + y
}

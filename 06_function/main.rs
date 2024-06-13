fn first() {
  let (x, y) = (1, 2);
  let s: i32 = sum(x, y);

  assert_eq!(s, 3);

  println!("Success!");
}

// funtions always have to annotate types for their arguments
fn sum(x: i32, y: i32) -> i32 {
  x + y
}

// --------------------------------------------

fn second() {
  print()
}

fn print() {
  println!("Success!");
}

// --------------------------------------------

fn third() {
  never_return();

  println!("Failed!");
}

// this is a diverging function
fn never_return() -> ! {
  panic!();
}

// --------------------------------------------

fn four() {
  println!("good");
}

fn get_option(tp: u8) -> Option<i32> {
  match tp {
    // if tp is 1 then do it
    1 => {
      // TODO
    }
    // anything else then do it
    _ => {
      // TODO
    }
  };

  // Rather that returning a None, we use a diverging function instead
  never_return_fn()
}

fn never_return_fn() -> ! {

  // three macros appropriate for the task
  
  // panic!()
  // unimplemented!()
  todo!()
}

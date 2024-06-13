fn main() {
  let e1: bool = false || true;
  let e2: bool = true && true || false;
  assert!(e1 == e2);

  print!("good");
}

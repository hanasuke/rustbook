fn hello() {
  // returnが書かれていないものはunit typeと解釈される
  println!("Hello");
}

fn main() {
  // returnがないものを束縛する
  let ret = hello();


  assert_eq!(ret, ()); // retがunit type`()`と一致する
  assert_eq!(std::mem::size_of::<()>(), 0); // メモリ上のサイズは0
}
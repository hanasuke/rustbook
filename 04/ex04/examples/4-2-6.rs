fn f1(mut n: u32) {
  // 呼び出し元の値のコピーをnに束縛している
  n += 1;
  println!("f1: n={}", n); // n = 6
}

fn f2(n_ptr: &mut u32) {
  // nは呼び出し元の値を指すアドレス
  println!("f2: n_ptr={:p}", n_ptr);

  *n_ptr = 2; // dereference
  println!("f2: n_ptr={}", *n_ptr);
}

fn main() {
  let mut n = 5;
  println!("main: n={}", n);

  f1(n);
  println!("main: n={}", n); // n = 5 (f1によって値は変更されない)

  f2(&mut n);
  println!("main: n={}", n); // n = 2 (f2によって値が変更される)
}
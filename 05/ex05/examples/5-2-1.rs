fn main() {
  // t1はスタック領域にデータを作る
  let t1 = (3, "birds".to_string());
  // Boxは，ヒープ領域に実データ，スタック領域にはヒープへのポインタを持つ
  let mut b1 = Box::new(t1);
  (*b1).0 += 1;
  assert_eq!(*b1, (4, "birds".to_string()));

  // t1の中身のデータはb1が所有権を持っているため，アクセスできない
  //println!("{:?}", t1);
}
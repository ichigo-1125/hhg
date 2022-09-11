各要素に対して引数で与えられたクロージャを実行し、trueを返す要素で区切られたスラ
イスのイテレータを返す。trueを返した要素は含まれない。

最初の要素および最後の要素に対する実行結果がtrueを返した場合、空のスライスが挿入
される。また、2つの隣り合う要素に対する実行結果がどちらもtrueであった場合には、
その間に空のスライスが挿入される。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let v = vector![10, 40, 33, 20];
let mut iter = v.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());

let u = [10, 40, 33];
let mut iter = u.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[]);
assert!(iter.next().is_none());

let w = [10, 6, 33, 20];
let mut iter = w.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10]);
assert_eq!(iter.next().unwrap(), &[]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

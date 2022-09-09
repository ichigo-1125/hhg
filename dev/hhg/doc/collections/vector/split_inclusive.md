各要素に対して引数で与えられたクロージャを実行し、trueを返す要素で区切られたスラ
イスのイテレータを返す。trueを返した要素は、前のスライスに含まれる。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let v = vector![10, 40, 33, 20];
let mut iter = v.split_inclusive(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

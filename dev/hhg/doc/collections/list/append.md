別のListを末尾に移動する。otherは空になる。

## 使用例

```
use hhg::collections::List;
use hhg::list;

let mut list = list![1, 2];
let mut list2 = list![3, 4];
list.append(&mut list2);

let mut iter = list.iter();
let mut iter2 = list2.iter();
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), Some(&4));
assert_eq!(iter.next(), None);
assert_eq!(iter2.next(), None);
```

先頭の要素を取得する。

## 使用例

```
use hhg::collections::list::List;
use hhg::list;

let mut list = list![1, 2, 3];
assert_eq!(list.pop_front(), Some(1));

let mut iter = list.iter();
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), None);
```

末尾の要素を取得する。

## 使用例

```
use hhg::collections::List;
use hhg::list;

let mut list = list![1, 2, 3];
assert_eq!(list.pop_back(), Some(3));

let mut iter = list.iter();
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), None);
```

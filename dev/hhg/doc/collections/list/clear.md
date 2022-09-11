全ての要素を削除する。ただし容量は変わらない。

## 使用例

```
use hhg::collections::List;
use hhg::list;

let mut list = list![1, 2, 3];
list.clear();
assert!(list.is_empty());
```

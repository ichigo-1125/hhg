先頭の要素の参照を取得する。

## 使用例

```
use hhg::collections::list::List;
use hhg::list;

let list = list![15, 31, 11, 25];
assert_eq!(list.front(), Some(&15));

let list2: List<i32> = list![];
assert_eq!(list2.front(), None);
```

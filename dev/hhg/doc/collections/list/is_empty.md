Listが空であるかを確認する。

## 使用例

```
use hhg::collections::list::List;
use hhg::list;

let list = list![15, 31, 11, 25];
assert_eq!(list.is_empty(), false);

let list2: List<i32> = List::new();
assert_eq!(list2.is_empty(), true);
```

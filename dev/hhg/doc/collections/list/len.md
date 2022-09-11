要素数を取得する。

## 使用例

```
use hhg::collections::List;
use hhg::list;

let list = list![15, 31, 11, 25];
assert_eq!(list.len(), 4);

let list2: List<i32> = List::new();
assert_eq!(list2.len(), 0);
```

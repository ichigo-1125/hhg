要素が存在するかを確認する。

## 使用例

```
use hhg::collections::List;

let mut list: List<u32> = List::new();

list.push_back(0);
list.push_back(1);
list.push_back(2);

assert_eq!(list.contains(&0), true);
assert_eq!(list.contains(&10), false);
```

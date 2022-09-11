指定したインデックスでListを分割し、末尾のListを返す。

## 使用例

```
use hhg::collections::List;
use hhg::list;

let mut d = list![1, 2, 3];
let mut split = d.split_off(2);

assert_eq!(split.pop_front(), Some(3));
assert_eq!(split.pop_front(), None);
```

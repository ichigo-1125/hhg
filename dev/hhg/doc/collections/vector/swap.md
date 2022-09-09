2つのインデックスを指定し、それらの要素を入れ替える。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector!["a", "b", "c", "d", "e"];
v.swap(2, 4);
assert!(v == ["a", "b", "e", "d", "c"]);
```

隣り合う要素のうち、keyを適用した結果が重複する要素（後ろのもの）を削除する。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![10, 20, 21, 30, 20];
v.dedup_by_key(|i| *i / 10);
assert_eq!(v, [10, 20, 30, 20]);
```

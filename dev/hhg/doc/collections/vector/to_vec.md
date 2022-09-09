スライス全体を複製して新しいVecを返す。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let v = vector![15, 31, 11, 25];
assert_eq!(v.to_vec(), vec![15, 31, 11, 25]);
```

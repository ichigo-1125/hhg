末尾の要素の可変参照を取得する。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![15, 31, 11, 25];
if let Some(last) = v.last_mut()
{
    *last = 0;
}
assert_eq!(v, &[15, 31, 11, 0]);
```

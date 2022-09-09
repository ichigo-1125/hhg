先頭の要素の可変参照を取得する。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![15, 31, 11, 25];
if let Some(first) = v.first_mut()
{
    *first = 0;
}
assert_eq!(v, &[0, 31, 11, 25]);
```

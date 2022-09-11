split_mutを末尾から実行する。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![100, 400, 300, 200, 600, 500];

let mut count = 0;
for group in v.rsplit_mut(|num| *num % 3 == 0)
{
    count += 1;
    group[0] = count;
}
assert_eq!(v, [3, 400, 300, 2, 600, 1]);
```

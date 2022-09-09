各要素に対して引数で与えられたクロージャを実行し、trueを返す要素で区切られた可変
スライスのイテレータを返す。trueを返した要素は含まれない。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![10, 40, 30, 20, 60, 50];

for group in v.split_mut(|num| *num % 3 == 0)
{
    group[0] = 1;
}
assert_eq!(v, [1, 40, 30, 1, 60, 1]);
```

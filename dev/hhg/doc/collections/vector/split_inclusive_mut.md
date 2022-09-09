各要素に対して引数で与えられたクロージャを実行し、trueを返す要素で区切られた可変
スライスのイテレータを返す。trueを返した要素は、前のスライスに含まれる。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![10, 40, 30, 20, 60, 50];

for group in v.split_inclusive_mut(|num| *num % 3 == 0)
{
    let terminator_idx = group.len()-1;
    group[terminator_idx] = 1;
}
assert_eq!(v, [10, 40, 1, 20, 1, 1]);
```

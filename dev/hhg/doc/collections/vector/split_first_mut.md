Vectorを最初の要素と残りの要素の可変スライスに分ける。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut x = vector![0, 1, 2];

if let Some((first, elements)) = x.split_first_mut()
{
    *first = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[3, 4, 5]);
```

Vectorを最後の要素と残りの要素の可変スライスに分ける。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut x = vector![0, 1, 2];

if let Some((last, elements)) = x.split_last_mut()
{
    *last = 3;
    elements[0] = 4;
    elements[1] = 5;
}
assert_eq!(x, &[4, 5, 3]);
```

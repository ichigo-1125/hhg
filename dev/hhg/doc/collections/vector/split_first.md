Vectorを最初の要素と残りの要素のスライスに分ける。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let x = vector![0, 1, 2];

if let Some((first, elements)) = x.split_first()
{
    assert_eq!(first, &0);
    assert_eq!(elements, &[1, 2]);
}
```

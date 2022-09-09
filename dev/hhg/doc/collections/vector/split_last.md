Vectorを最後の要素と残りの要素のスライスに分ける。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let x = vector![0, 1, 2];

if let Some((last, elements)) = x.split_last()
{
    assert_eq!(last, &2);
    assert_eq!(elements, &[0, 1]);
}
```

与えられたchunk_sizeごとに要素を分割し、その可変スライスのイテレータを返す。
chunk_sizeに満たないチャンクはイテレータには含まれず、remainder()で取得できる。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let mut v = vector![0, 0, 0, 0, 0];
let mut count = 1;

for chunk in v.chunks_exact_mut(2)
{
    for elem in chunk.iter_mut()
    {
        *elem += count;
    }
    count += 1;
}
assert_eq!(v, &[1, 1, 2, 2, 0]);
```

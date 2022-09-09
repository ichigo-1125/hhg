与えられたchunk_sizeごとに要素を分割し、そのスライスのイテレータを返す。
chunk_sizeに満たないチャンクはイテレータには含まれず、remainder()で取得できる。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let v = vector!['l', 'o', 'r', 'e', 'm'];
let mut iter = v.chunks_exact(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert!(iter.next().is_none());
assert_eq!(iter.remainder(), &['m']);
```

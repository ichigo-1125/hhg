Dequeの内容を連続したメモリ領域に配置し直し、スライスを返す。

## 使用例

```
use hhg::collections::Deque;

let mut buf = Deque::with_capacity(15);

buf.push_back(2);
buf.push_back(1);
buf.push_front(3);

buf.make_contiguous().sort();
assert_eq!(buf.as_slices(), (&[1, 2, 3] as &[_], &[] as &[_]));

buf.make_contiguous().sort_by(|a, b| b.cmp(a));
assert_eq!(buf.as_slices(), (&[3, 2, 1] as &[_], &[] as &[_]));
```

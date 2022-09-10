要素数に合わせて容量を縮小する。

## 使用例

```
use hhg::collections::deque::Deque;

let mut deq = Deque::with_capacity(10);
deq.extend([1, 2, 3]);
assert!(deq.capacity() >= 10);
deq.shrink_to_fit();
assert_eq!(deq.capacity(), 3);
```

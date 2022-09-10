容量を確保する。将来の再確保に備えて余分に容量を確保することはない。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![1];
deq.reserve_exact(10);
assert!(deq.capacity() >= 11);
```

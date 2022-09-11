容量を確保する。将来の再確保に備えて余分に容量を確保する可能性がある。

## 使用例

```
use hhg::collections::Deque;
use hhg::deque;

let mut deq = deque![1];
deq.reserve(10);
assert!(deq.capacity() >= 11);
```

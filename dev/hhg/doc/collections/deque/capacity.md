容量を取得する。

## 使用例

```
use hhg::collections::Deque;

let deq: Deque<i32> = Deque::with_capacity(10);
assert!(deq.capacity() >= 10);
```

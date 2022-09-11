指定したインデックスでDequeを分割し、末尾のDequeを返す。

## 使用例

```
use hhg::collections::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3];
let mut deq2 = deq.split_off(1);
assert_eq!(deq, [1]);
assert_eq!(deq2, [2, 3]);
```

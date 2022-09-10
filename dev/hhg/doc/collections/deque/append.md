別のDequeを末尾に移動する。otherは空になる。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3];
let mut deq2 = deque![4, 5, 6];
deq.append(&mut deq2);
assert_eq!(deq, [1, 2, 3, 4, 5, 6]);
assert_eq!(deq2, []);
```

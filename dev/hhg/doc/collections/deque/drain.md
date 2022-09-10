指定された範囲を一括で削除し、削除された要素をイテレータとして返す。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3];
let deq2: Deque<_> = deq.drain(1..).collect();
assert_eq!(deq, [1]);
assert_eq!(deq2, [2, 3]);

deq.drain(..);
assert_eq!(deq, []);
```

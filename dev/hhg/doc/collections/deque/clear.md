全ての要素を削除する。ただし容量は変わらない。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3];
deq.clear();
assert!(deq.is_empty());
```

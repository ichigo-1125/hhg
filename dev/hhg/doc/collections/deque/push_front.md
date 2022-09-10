先頭に要素を追加する。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![1, 2];
deq.push_front(3);
assert_eq!(deq, [3, 1, 2]);
```

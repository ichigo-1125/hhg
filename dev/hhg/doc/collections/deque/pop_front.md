先頭の要素を取得する。

## 使用例

```
use hhg::collections::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3];
assert_eq!(deq.pop_front(), Some(1));
assert_eq!(deq, [2, 3]);
```

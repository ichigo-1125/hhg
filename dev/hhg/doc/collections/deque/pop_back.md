末尾の要素を取得する。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3];
assert_eq!(deq.pop_back(), Some(3));
assert_eq!(deq, [1, 2]);
```

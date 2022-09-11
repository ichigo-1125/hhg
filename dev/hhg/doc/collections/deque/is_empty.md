Dequeが空であるかを確認する。

## 使用例

```
use hhg::collections::Deque;
use hhg::deque;

let deq = deque![15, 31, 11, 25];
assert_eq!(deq.is_empty(), false);

let deq: Deque<i32> = Deque::new();
assert_eq!(deq.is_empty(), true);
```

要素数を取得する。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let deq = deque![15, 31, 11, 25];
assert_eq!(deq.len(), 4);

let deq2: Deque<i32> = Deque::new();
assert_eq!(deq2.len(), 0);
```

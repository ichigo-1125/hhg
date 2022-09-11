先頭の要素の参照を取得する。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let deq = deque![15, 31, 11, 25];
assert_eq!(deq.front(), Some(&15));

let deq2: Deque<i32> = deque![];
assert_eq!(deq2.front(), None);
```

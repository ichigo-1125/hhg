指定したインデックスに要素を挿入する。それ以降の要素は右シフトする。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3];
deq.insert(1, 4);
assert_eq!(deq, [1, 4, 2, 3]);
deq.insert(4, 5);
assert_eq!(deq, [1, 4, 2, 3, 5]);
```

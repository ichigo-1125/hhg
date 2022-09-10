指定したインデックスの要素の参照を取得する。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let deq = deque![15, 31, 11, 25];
assert_eq!(deq.get(2), Some(&11));
assert_eq!(deq.get(8), None);
```

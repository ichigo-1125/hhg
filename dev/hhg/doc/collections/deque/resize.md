Dequeをリサイズする。

new_lenが現在の長さよりも小さかった場合、new_lenに合わせて末尾の要素が切り捨てら
れる。new_lenが現在の長さよりも大きかった場合、Dequeは拡張され、valueで埋められ
る。

## 使用例

```
use hhg::collections::Deque;
use hhg::deque;

let mut deq = deque![15, 31, 11, 25];

deq.resize(6, 20);
assert_eq!(deq, [15, 31, 11, 25, 20, 20]);

deq.resize(3, 20);
assert_eq!(deq, [15, 31, 11]);
```

Dequeをリサイズする。

new_lenが現在の長さよりも小さかった場合、new_lenに合わせて末尾の要素が切り捨てら
れる。new_lenが現在の長さよりも大きかった場合、Dequeは拡張され、valueで埋められ
る。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3];
deq.resize_with(5, Default::default);
assert_eq!(deq, [1, 2, 3, 0, 0]);

let mut deq2 = deque![];
let mut p = 1;
deq2.resize_with(4, || { p *= 2; p });
assert_eq!(deq2, [2, 4, 8, 16]);
```

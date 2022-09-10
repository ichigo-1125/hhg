指定されたlenまでDequeを縮小する。lenが現在の要素数よりも小さい場合、末尾の要素
を削除する。ただし容量は変わらない。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3, 4];
deq.truncate(2);
assert_eq!(deq, [1, 2]);

let mut deq2 = deque![1, 2, 3];
deq2.truncate(8);
assert_eq!(deq2, [1, 2, 3]);

let mut deq3 = deque![1, 2, 3];
deq3.truncate(0);
assert_eq!(deq3, []);
```

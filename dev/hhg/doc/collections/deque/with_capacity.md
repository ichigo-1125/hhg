容量を指定して空のDequeを生成する。

## パラメータ

* capacity : あらかじめ確保する容量

## 使用例

```
use hhg::collections::deque::Deque;

let deq: Deque<i32> = Deque::with_capacity(10);
```

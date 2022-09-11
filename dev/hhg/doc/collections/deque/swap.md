2つのインデックスを指定し、それらの要素を入れ替える。

## 使用例

```
use hhg::collections::Deque;
use hhg::deque;

let mut deq = deque!["a", "b", "c", "d", "e"];
deq.swap(2, 4);
assert!(deq == ["a", "b", "e", "d", "c"]);
```

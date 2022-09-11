各要素に対して引数で与えられたクロージャを実行し、falseを返す全ての要素を削除す
る。

## 使用例

```
use hhg::collections::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3, 4];
deq.retain(|&x| x % 2 == 0);
assert_eq!(deq, [2, 4]);
```

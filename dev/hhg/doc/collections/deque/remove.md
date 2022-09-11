指定したインデックスの要素を削除して返す。それ以降の要素は左シフトする。順番を保
持しなくてよい場合はswap_remove_frontやswap_remove_backの方が高速。

## 使用例

```
use hhg::collections::Deque;
use hhg::deque;

let mut deq = deque![1, 2, 3];
assert_eq!(deq.remove(1), Some(2));
assert_eq!(deq, [1, 3]);
```

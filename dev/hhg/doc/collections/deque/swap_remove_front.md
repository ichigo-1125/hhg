要素を削除して返す。削除された要素はDequeの最初の要素に置き換えられる。順番は保
持しないがremoveより高速。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque!["foo", "bar", "baz", "qux"];

assert_eq!(deq.swap_remove_front(1), Some("bar"));
assert_eq!(deq, ["foo", "baz", "qux"]);

assert_eq!(deq.swap_remove_front(0), Some("foo"));
assert_eq!(deq, ["baz", "qux"]);
```

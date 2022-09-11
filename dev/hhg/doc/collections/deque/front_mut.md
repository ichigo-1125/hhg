先頭の要素の可変参照を取得する。

## 使用例

```
use hhg::collections::deque::Deque;
use hhg::deque;

let mut deq = deque![15, 31, 11, 25];
if let Some(first) = deq.front_mut()
{
    *first = 0;
}
assert_eq!(deq, &[0, 31, 11, 25]);
```

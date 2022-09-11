末尾の要素の可変参照を取得する。

## 使用例

```
use hhg::collections::Deque;
use hhg::deque;

let mut deq = deque![15, 31, 11, 25];
if let Some(last) = deq.back_mut()
{
    *last = 0;
}
assert_eq!(deq, &[15, 31, 11, 0]);
```

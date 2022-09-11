末尾の要素の可変参照を取得する。

## 使用例

```
use hhg::collections::list::List;
use hhg::list;

let mut list = list![15, 31, 11, 25];
if let Some(last) = list.back_mut()
{
    *last = 0;
}

let mut iter = list.iter();
assert_eq!(iter.next(), Some(&15));
assert_eq!(iter.next(), Some(&31));
assert_eq!(iter.next(), Some(&11));
assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), None);
```

先頭の要素の可変参照を取得する。

## 使用例

```
use hhg::collections::List;
use hhg::list;

let mut list= list![15, 31, 11, 25];
if let Some(first) = list.front_mut()
{
    *first = 0;
}

let mut iter = list.iter();
assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&31));
assert_eq!(iter.next(), Some(&11));
assert_eq!(iter.next(), Some(&25));
assert_eq!(iter.next(), None);
```

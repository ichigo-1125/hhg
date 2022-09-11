要素を検索する。要素が見つかった場合はそのインデックスを返し、見つからなかった場
合は、ソートの順番を維持した場合に検索値が挿入されるべきインデックスを返す。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let s = vector![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

assert_eq!(s.binary_search(&13), Ok(9));
assert_eq!(s.binary_search(&4), Err(7));
assert_eq!(s.binary_search(&100), Err(13));
let r = s.binary_search(&1);
assert!(match r { Ok(1..=4) => true, _ => false, });
```

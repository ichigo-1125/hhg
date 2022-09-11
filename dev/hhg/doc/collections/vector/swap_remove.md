要素を削除して返す。削除された要素はVectorの最後の要素に置き換えられる。順番は保
持しないがremoveより高速。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector!["foo", "bar", "baz", "qux"];

assert_eq!(v.swap_remove(1), "bar");
assert_eq!(v, ["foo", "qux", "baz"]);

assert_eq!(v.swap_remove(0), "foo");
assert_eq!(v, ["baz", "qux"]);
```

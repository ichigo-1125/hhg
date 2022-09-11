指定したインデックスの要素を削除して返す。それ以降の要素は左シフトする。順番を保
持しなくてよい場合はswap_removeの方が高速。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector![1, 2, 3];
assert_eq!(v.remove(1), 2);
assert_eq!(v, [1, 3]);
```

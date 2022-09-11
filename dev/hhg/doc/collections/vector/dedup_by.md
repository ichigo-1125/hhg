隣り合う要素のうち、指定された等値関係を満たす要素を削除する。same_bucketには、
ベクトルから2つの要素への参照が渡され、要素が等しいかどうかを判定した結果を返す
必要がある。要素はスライス内の順と逆の順序で渡されるため、same_bucket(a, b)が
trueを返す場合は、aが削除される点に注意。

## 使用例

```
use hhg::collections::Vector;
use hhg::vector;

let mut v = vector!["foo", "bar", "Bar", "baz", "bar"];
v.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
assert_eq!(v, ["foo", "bar", "baz", "bar"]);
```

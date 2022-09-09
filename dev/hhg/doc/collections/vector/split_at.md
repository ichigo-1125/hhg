指定したインデックスでVectorを2つのスライスに分ける。元のVectorは変更されない。

## 使用例

```
use hhg::collections::vector::Vector;
use hhg::vector;

let v = vector![1, 2, 3, 4, 5, 6];

{
    let (left, right) = v.split_at(0);
    assert_eq!(left, []);
    assert_eq!(right, [1, 2, 3, 4, 5, 6]);
}

{
    let (left, right) = v.split_at(2);
    assert_eq!(left, [1, 2]);
    assert_eq!(right, [3, 4, 5, 6]);
}

{
    let (left, right) = v.split_at(6);
    assert_eq!(left, [1, 2, 3, 4, 5, 6]);
    assert_eq!(right, []);
}
```

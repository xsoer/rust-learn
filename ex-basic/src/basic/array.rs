// array
// 数组类型
// A list with each element, i.e., [x, y, z].
// A repeat expression [x; N], which produces an array with N copies of x. The type of x must be Copy.
// 相同类型固定大小的若干元素集合
// 数组的类型是[T; N].
// 是否是固定排序？

// 默认数据类型: i32
// let a = [10; 10];
// let b = [5; 10];
// a.len()
// a.iter()
// a.contains(&T)
// a.map(|x| x+1)
// a.zip(b)
// a[1]     数组存取是在运行时进行边界检查的

//
fn init() {
    let mut array: [i32; 3] = [0; 3];
    let b = [0; 3];
    println!("{:?}", array);
    println!("b = {:?}", b);

    array[1] = 1;
    array[2] = 2;

    // as slice
    assert_eq!([1, 2], &array[1..]);

    // This loop prints: 0 1 2
    // for x in array 是不行的。
    for x in &array {
        print!("{} ", x);
    }
}

// 判断数组中是否包含不合法值
fn exist_false() {
    let a = [0, 1, 3];
    let b = [1, 2, 3];
    // 迭代过滤判断
    if a.iter().filter(|&x| x == &0).count() != 0 {
        println!("a has zero")
    } else {
        println!("a no zero")
    }

    if b.iter().filter(|&x| x == &0).count() != 0 {
        println!("b has zero")
    } else {
        println!("b has no zero")
    }

    // 判断包含过滤
    if a.contains(&0) {
        println!("a has zero")
    } else {
        println!("a no zero")
    }

    if b.contains(&0) {
        println!("b has zero")
    } else {
        println!("b has no zero")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        init()
    }

    #[test]
    fn test_exist_false() {
        exist_false()
    }
}

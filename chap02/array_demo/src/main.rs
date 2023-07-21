use std::mem;

// 此函数借用一个slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len())
}

fn main() {
    // 定长数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素初始化相同的值
    let ys: [i32; 10] = [3; 10];
    // 下标从0开始
    println!("ys first elem: {}", ys[0]);
    println!("ys last elem: {}", ys[9]);

    // 数组的长度
    println!("size of xs is {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupise {} bytes", mem::size_of_val(&xs));

    // 数组可自动被借用成slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // 越界下标会引发致命错误panic
    // println!("{}", xs[5])
}

fn main() {
    // Sum 
    let v = [1,2,3];
    let total: i32 = v.iter().sum();
    println!("total: {}", total);

    // Any
    let v = [1,3,4,5];
    let result1 = v.iter().any(|&x| x == 2);
    let result2 = v.iter().any(|x| *x == 2);

    //line 13 会发生错误： 消费器any的内部实现中有一个for循环，会自动调用迭代器的next方法
    //返回Option<&[T]> 或 Option<&mut[T]>类型的值，再通过模式匹配得刀&[T] 或 &mut[T]类型的值
    //因此，在消费器any调用闭包中只有使用引用和解引用两种方式
    // let result3 = v.iter().any(|x| x == 2);

    print!("{}", result1);
    print!("{}", result2);

    // Collect
    let v1 = [1,2,3,4,5];
    let v2 : Vec<i32> = v1.iter().map(|x| x+1).collect();

    println!("{:?}", v2);
}

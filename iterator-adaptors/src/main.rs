fn main() {
    // map
    //适配器map对迭代器中每个元素调用闭包并生成_个新迭代器
    let v = [1,2,3];
    let result: Vec<i32> = v.iter()
        .map(|x| x + 3)
        .collect();

    println!("{:?}",result);

    // take
    //适配器take生成_个仅迭代原迭代器中前"个元素的新迭代器’常用于遍历指定数量
    //元素的场景°代码清单4ˉ13中’迭代器使用适配器take生成_个由前3个元素组成的新迭代器°
    let v = [1,2,3,4,5];
    let result = v.iter().take(3);

    for i in result {
        println!("{}", i);
    }

    // filter
    //适配器hlter对迭代器中每个元素调用闭包并生成_个过滤元素的新迭代器°闭
    //包会返 回tme或false’如果返回true则该元素放人新迭代器,否则该元素将被忽略。
    let v = [1,2,3];
    let result: Vec<i32> = v.iter()
        .map(|x| x + 3)
        .filter(|x| x % 3 == 0)
        .collect();

    println!("{:?}", result);


    //rev
    //通常,迭代器从左到右进行迭代°适配器rev可以反转迭代方向’生成_个方向相反的迭代器
    let v = [1,2,3];
    let result = v.iter().rev();

    

}

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
    //适配器filter对迭代器中每个元素调用闭包并生成_个过滤元素的新迭代器°闭
    //包会返 回true或false’如果返回true则该元素放人新迭代器,否则该元素将被忽略。
    let v = [1,2,3];
    let result: Vec<i32> = v.iter()
        .map(|x| x + 3)
        .filter(|x| x % 3 == 0)
        .collect();

    println!("{:?}", result);
    println!("{:?}", result);
    //result: [6]


    //rev
    //通常,迭代器从左到右进行迭代°适配器rev可以反转迭代方向’生成_个方向相反的迭代器
    let v = [1,2,3];
    let result = v.iter().rev();

    for i in result {
        println!("{}", i);
    }
    //result:
    //3
    //2
    //1


    // zip
    // 适配器Zip可将两个迭代器压缩在_起生成_个新迭代器。实际上’它在两个迭代器上
    // 同时迭代并返回_个元组’其中第_个元素来自第_个迭代器,第二个元素来自第二个迭 
    // 代器°如果两个迭代器中任_迭代器返回None’适配器zip就返回None。
    let v1 = [1,2,3];
    let v2 = [2,3,6];

    let result: Vec<i32> = v1.iter().zip(v2.iter())
        .map(|(a, b)| a + b)
        .filter(|x| x % 3 == 0)
        .collect();

    println!("{:?}", result);

    //result: [3, 9]
}

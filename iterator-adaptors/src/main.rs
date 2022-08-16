fn main() {
    // map
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
}

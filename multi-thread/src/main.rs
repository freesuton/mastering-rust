use std::time::Duration;

fn main() {

 let mut num = Vec::with_capacity(11);

 //线程中使用了其他线程的变量是不合法的，必须使用move表明线程拥有data的所有权
 let a = {
    let a = std::thread::spawn(move || {
        for i in 0..10 {
           num.push(i);
        }
        num
    });
    a
 };

 let b = a.join().unwrap();
 {
     std::thread::spawn(move || {
         let b = b.clone();
         b.push(0);
     });
    // println!("{}", b.capacity());
 }
}

// fix thread issue
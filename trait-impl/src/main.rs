mod trait_impl;
use self::trait_impl::Geometry;


fn main() {
    let rect = trait_impl::Rectangle {width: 8.8, height: 2.2};
    println!("rect.area: {}, rect.perimeter: {}",
        rect.area(), rect.perimeter());

    let circle = trait_impl::Circle { radius: 3.0 };
    println!("circle.area: {}, circle.perimeter: {}",
        circle.area(), circle.perimeter());



}

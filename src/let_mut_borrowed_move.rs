#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn change(m: &mut Point) {
    m.x = 5;
}

fn main() {
    let mut p1 = Point { x: 10, y: 10 };
    let p2 = &mut Point { x: 10, y: 10 };

    change(&mut p1);
    change(p2);

    let p3 = p1;
    let p4 = p2;

    println!("p1={:#?}", p3);
    println!("p2={:#?}", p4);

    println!("p1==p2->{:#?}", p3 == *p4);
}

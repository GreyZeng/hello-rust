struct Point {
    x: i32,
    y: i32,
}

fn simple_borrow() {
    let mut p1 = Point { x: 25, y: 24 };
    f(&mut p1); // p1的所有权转移到p，引出借用的概念
    println!("{} {}", p1.x, p1.y);
}

// 只读借用,如果需要修改，一定要加mut
fn f(p: &mut Point) {
    p.x = 3;
}

fn main() {
    simple_borrow();
}

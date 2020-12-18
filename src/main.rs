use std::thread;
use std::time::Duration;
use std::rc::Rc;

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

fn new_thread() {
    let p1 = Point { x: 245, y: 23 };
    let h = thread::spawn(move || {
        for i in 0..10 {
            // thread::sleep(Duration::from_millis(100));
            println!("{} {}", p1.x, p1.y);
        }
    });

    h.join();
}

fn main() {
    // Box保证分配到堆,只能有一个所有权人
    let p = Box::new(Point { x: 23, y: 32 });
    println!("{}", (*p).x);
    println!("{}", p.x);
    // new_thread();

    let p2 = Rc::new(Point { x: 3, y: 34 });
    let p3 = Rc::clone(&p2);
    let p4 = Rc::clone(&p2);
    println!("{} {} {}", p2.x, p3.x, p4.x);
    // simple_borrow();
}

use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

struct Point {
    x: i32,
    y: i32,
}
fn hello() {
    println!("hello, world");
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

fn mutex_thread() {
    let mut p_arc_mux = Arc::new(Mutex::new(Point { x: 1, y: 3 }));

    for _ in 0..10 {
        let pp = Arc::clone(&p_arc_mux);
        let h = thread::spawn(move || {
            let mut p = pp.lock().unwrap();
            p.x += 1;
        });
        h.join();
    }


    let p1 = Arc::clone(&p_arc_mux);
    let h = thread::spawn(move || {
        let mut p = p1.lock().unwrap();
        for i in 0..10 {
            // thread::sleep(Duration::from_millis(100));

            p.x += 1;
            p.y += 1;
        }
        println!("{} {}", p.x, p.y);
    });

    h.join();
    let p2 = Arc::clone(&p_arc_mux);
    let h2 = thread::spawn(move || {
        let mut p = p2.lock().unwrap();
        for i in 0..10 {
            // thread::sleep(Duration::from_millis(100));

            p.x += 1;
            p.y += 1;
        }
        println!("{} {}", p.x, p.y);
    });

    h2.join();
    /* let mut  p = p_mux.lock().unwrap();
     p.x+=1;
     println!("{}",p.x);*/
}

fn multi_thread() {
    let p1 = Arc::new(Point { x: 3, y: 4 });
    let p2 = Arc::clone(&p1);
    let h = thread::spawn(move || {
        println!("{} {}", p2.x, p2.y);
    });
    let p3 = Arc::clone(&p1);
    let h2 = thread::spawn(move || {
        println!("{} {}", p3.x, p3.y);
    });
    h.join();
    h2.join();
}

fn main() {
    // Box保证分配到堆,只能有一个所有权人
    /* let p = Box::new(Point { x: 23, y: 32 });
     println!("{}", (*p).x);
     println!("{}", p.x);
     // new_thread();

     let p2 = Rc::new(Point { x: 3, y: 34 });
     let p3 = Rc::clone(&p2);
     let p4 = Rc::clone(&p2);
     println!("{} {} {}", p2.x, p3.x, p4.x);*/

    //mutex_thread();
    //multi_thread();
    // simple_borrow();
    hello()
}

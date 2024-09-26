use std::thread;
use thread_local::ThreadLocal;
use std::sync::Arc;
use std::cell::Cell;

fn main(){
    // 线程安全的本地存储 TLS
    let tls = Arc::new(ThreadLocal::new());

    let mut handlers = vec![];

    for i in 0..16 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            let cell = tls2.get_or(|| Cell::new(0));

            println!("thread {} get value {}", i, cell.get());
            cell.set(cell.get() + 1);
            println!("thread {} set value {}", i, cell.get());

        });

        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        println!("x:{} , y: {}", x, y.get());
        x + y.get()
    });

    println!("The value of total = {}", total);
}
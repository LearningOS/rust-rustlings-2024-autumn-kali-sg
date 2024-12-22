// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// Rust 的标准库中的通道（std::sync::mpsc）本身就是线程安全的，设计上允许多个生产者（sender）和一个消费者（receiver）。
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    /* mpsc::Sender<u32> 类型并没有实现 Copy 特性，所以当你第一次在闭包中使用 move || { ... tx.send(*val) ... } 时，
     * tx 被移动到了第一个线程中。然后当第二个线程试图再次使用 tx 时，编译器就会报错，因为 tx 已经被移动且不能再用。
     * 要解决这个问题，你可以使用 mpsc::channel 函数返回的 Sender 来创建多个发送者。Sender 实现了 Clone 特性，
     * 因此你可以通过调用 clone() 方法来复制 Sender，从而让每个线程都有自己的 Sender 实例。 */

     let tx1 = tx.clone();  // 对原有的tx进行复制，防止所有权转移导致错误
     let tx2 = tx.clone();
 

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}

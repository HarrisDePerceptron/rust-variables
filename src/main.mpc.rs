use std::io::Error;

use std::thread;
use std::sync::mpsc::channel;



fn main() {
    println!("hello world ");
    
    let (tx, rx) = channel();

    for i in 0..10 {
        let tx = tx.clone();

        thread::spawn(move ||{
            let du = std::time::Duration::new(i,0);
            thread::sleep(du);
            tx.send(i).unwrap();
    
        });

    }

    loop {
        println!("Waiting to resv...");
        
        let got = rx.recv().unwrap();
        println!("Got {got}");
    }
    
}

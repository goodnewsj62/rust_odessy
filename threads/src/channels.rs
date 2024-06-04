use std::{thread, time::Duration};
pub use std::sync::mpsc;

pub fn ts(){
    let (tx, rx)=  mpsc::channel();

    thread::spawn(move||{
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received =  rx.recv().unwrap();
    println!("Got:  {received}");
}
pub fn pause_send(){
    let (tx, rx)=  mpsc::channel();

    thread::spawn(move||{
        let vals =vec![ String::from("hi"),  String::from("how"),  String::from("are"),  String::from("you")];
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

   for received in rx{
    println!("Got:  {received}");
   }
}
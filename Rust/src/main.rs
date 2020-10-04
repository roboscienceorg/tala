
#![allow(non_snake_case)]
mod TEST_Channel;
mod channel;
mod master;
use std::thread;


fn main() {
    //println!("ChannelTests");

    TEST_Channel::test();


    //let m = master::Master::new();
    //let m = master::Master {ipAddress: "127.0.0.1".to_string(), port: 10819};

    let ip = "127.0.0.1".to_string();
    let port = 25565;
    let m = master::connect(ip, port);
    
    
    let sub_ = m.subscriber();
    let sub_2 = m.subscriber();
    let pub_ = m.publisher();

    m.host();

    println!("Back to main from hosting");
    //code

    let mut line = String::new();
    println!("Break1:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();


    m.terminate();

    let mut xx = String::new();
    println!("Break2:");
    let b1 = std::io::stdin().read_line(&mut xx).unwrap();

    println!("Finished mains");
    /*
    let mut publisher = m.publisher();
    let channel = "X92.FM".to_string();
    let message = "FirstMessage".to_string();
    publisher.connect(channel);
    publisher.publish(channel, message);

    let mut subscriber = m.subscriber();
    let data = subscriber.listen(channel);

    println!("{} = FirstMessage",data.to_string());
    */
    
}

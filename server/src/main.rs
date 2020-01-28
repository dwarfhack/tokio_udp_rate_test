#![warn(rust_2018_idioms)]

use structopt::StructOpt;

use std::error::Error;
use tokio;
use tokio::net::UdpSocket;

use std::time::SystemTime;

use config::Opt;
use netcode::packets::*;
use tokio::runtime::Runtime;

use tokio::sync::mpsc;
use tokio::sync::mpsc::*;
use futures::future::join_all;

use log::{info, warn};

mod config;

pub fn main(){
    simple_logger::init().unwrap();

    let opt : Opt = Opt::from_args();

    println!("{:#?}", opt);

    let mut rt = Runtime::new().unwrap();

    let (mut tx,rx) = mpsc::channel(100);

    let handler = rt.spawn(rcv_pass_handler(opt.clone(),rx));
    let receiver = rt.spawn(rcv_pass(opt.clone(),tx));
    let join_handler = join_all(vec![handler,receiver]);

    rt.block_on(join_handler);

}

async fn rcv_pass(opt: Opt,mut tx: Sender<MovementUpdate>,) {

    let mut socket = UdpSocket::bind(&opt.bind_addr).await.unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    let ms_in_sec=1_000.0;

    let mut buf= vec![0; 512];

    let mut msg_ctr = 0;
    let mut total_msg_ctr = 0;

    let mut start_time = SystemTime::now();

    loop{
        let res = socket.recv_from(&mut buf).await.unwrap();

        // reset start time when we start to receive messages
        if msg_ctr == 0 {
            start_time = SystemTime::now();
        }

        let packet: MovementUpdate = bincode::deserialize(&buf[..]).unwrap();

        tx.send(packet).await.unwrap();

        msg_ctr+=1;
        total_msg_ctr+=1;
        if msg_ctr%1_000_000 == 1_000_000-1 {
            let timediff = start_time.elapsed().unwrap();
            let packet_rate = (1_000_000.0/timediff.as_millis()as f32)*ms_in_sec;

            info!("[R] Current packet rate {}Pps at count {}",packet_rate,total_msg_ctr);
            start_time = SystemTime::now();
        }
    }

}

async fn rcv_pass_handler(opt: Opt,mut rx: Receiver<MovementUpdate>) {
    let mut msg_ctr = 0;
    let mut total_msg_ctr = 0;
    let mut start_time = SystemTime::now();

    while let Some(packet) = rx.recv().await {
        msg_ctr+=1;
        total_msg_ctr+=1;

        if msg_ctr%1_000_000 == 1_000_000-1 {
            let timediff = start_time.elapsed().unwrap();
            let packet_rate = (1_000_000.0/timediff.as_millis()as f32)*1_000.0;
            info!("[H] Current packet rate {}Pps at count {}",packet_rate,total_msg_ctr);
            start_time = SystemTime::now();
        }
    }

}

async fn rcv_drop(opt: Opt) -> Result<(), Box<dyn Error>> {

    let mut socket = UdpSocket::bind(&opt.bind_addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let mut buf= vec![0; 512];
    let mut msg_ctr = 0;
    let mut start_time = SystemTime::now();

    loop{
        let res = socket.recv_from(&mut buf).await?;

    // reset start time when we start to receive messages to rule out client startup delay
    if msg_ctr == 0 {
            start_time = SystemTime::now();
        }

        msg_ctr+=1;
        if msg_ctr%1_000_000==1_000_000-1 {
            let timediff = start_time.elapsed().unwrap();
            let packet_rate = (1_000_000.0/timediff.as_millis()as f32)*1_000f32;

            info!("Current packet rate {}Pps",packet_rate);

            start_time = SystemTime::now();
        }
    }
    Ok(())
}
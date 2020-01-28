
use structopt::StructOpt;

use std::error::Error;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

use netcode::packets::MovementUpdate;

use config::Opt;

use std::time::{SystemTime};

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let opt :Opt = Opt::from_args();


    let remote_addr: SocketAddr = opt.connect_addr.parse()?;
    let local_addr: SocketAddr = opt.bind_addr.parse()?;


    let mut socket = UdpSocket::bind(local_addr).await?;
    socket.connect(&remote_addr).await?;


    let mut last_sent_time = SystemTime::now();

    // want to have 100K msg/sec
    // -> 10us per msg


    /*
        We want to send the same update all the time 
        so we can construct it outside the loop
    */
    let update = MovementUpdate{
        id: 7
    };
    let encoded = bincode::serialize(&update).unwrap();

    for _k in 0 .. 10_000_000 {

        while last_sent_time.elapsed().unwrap().as_micros() < 10 {
            // spinloop
        }
        last_sent_time = SystemTime::now();

        socket.send(&encoded).await?;
    }


    Ok(())
}

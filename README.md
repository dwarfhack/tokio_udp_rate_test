# Little benchmark for Rust + Tokio UDP pps performance

To find out how many URP packets we can send, this project does the following:

- The server receives UDP packets at a given port
- The client sends tiny updates at a fixe rate (atm 100K per second)
- The server receives the datagrams, parses them to the original rust struct
- The parsed packets are send via a `tokio::sync::mpsc` channel to another task for processing.


## Running
- run the server: `cd server && cargo run --release`
- run the client (possibly on a different machine): `cd client && ./run_many_clients.sh`

The server will then display the rate and cound of packets received

## Results

Between two hosts I could achieve about 500Kpps.
Increasing the client count or rate did not increase the processing rate. 
When starting multiple independent servers, the total processing rate did not increase either.
I therefore conclude that these 500Kpps are limited by the OS or hardware.
This assumption is supported by the fact, that if running on localhost, I can acheive about 700Kpps on my machine.

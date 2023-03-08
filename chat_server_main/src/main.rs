use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    //first lests set up a listerner
    let listerner = TcpListener::bind("localhost:8080").await.unwrap();
    //time to start accepting connectiion.

    // client connected need to chat with each other
    let (tx, rx) = broadcast::channel(10);
    loop {
        let (mut socket, addr) = listerner.accept().await.unwrap();

        /* loop{

             // what we do now is read something from client and write somwthing back
             //read memory from a network stream , we need a buffer to but that in'
             let mut buffer:[u8; 1024] = [0; 1024];

             //use byte read to truncate our buffer when we send to a client

            let byte_read =  socket.read(&mut buffer).await.unwrap();
            socket.write_all(&buffer[..byte_read]).await.unwrap();

            //server has echoed here


        } */

        //more idiomatic rust. truncating the buffer that way dont look too idiomatic

        let tx = tx.clone();
        let mut rx = tx.subscribe(); // creates a new receiver

        tokio::spawn(async move {
            let (read_half, mut write_half) = socket.split();
            let mut reader = BufReader::new(read_half);
            let mut line = String::new();
            loop {
                //we need a string to store each line

                //our message logic initially is wack. sellect macro help in only returning whrn the first concurent task completes
                tokio::select! {

                    result = reader.read_line(&mut line) =>{
                         if result.unwrap() == 0 {
                    break;
                }

                tx.send((line.clone(), addr)).unwrap();
                line.clear();

                    }

                    result = rx.recv() =>{
                        let (message, other_addr) = result.unwrap();

                        if addr != other_addr{
                        write_half.write_all(message.as_bytes()).await.unwrap();
                        }

                    }

                }

                // line.clear(); // avoid some repetiton on the cmd
            }
        });
    }
}

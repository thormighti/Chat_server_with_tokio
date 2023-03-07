use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    //first lests set up a listerner
    let listerner = TcpListener::bind("localhost:8080").await.unwrap();
    //time to start accepting connectiion.
    loop {
        let (mut socket, _) = listerner.accept().await.unwrap();

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

        tokio::spawn(async move {
            let (read_half, mut write_half) = socket.split();
            let mut reader = BufReader::new(read_half);
            let mut line = String::new();
            loop {
                //we need a string to store each line

                let byte_read = reader.read_line(&mut line).await.unwrap();
                if byte_read == 0 {
                    break;
                }

                write_half.write_all(line.as_bytes()).await.unwrap();
                line.clear(); // avoid some repetiton on the cmd
            }
        });
    }
}

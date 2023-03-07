use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() {
    //first lests set up a listerner
    let listerner = TcpListener::bind("localhost:8080").await.unwrap();

    //time to start accepting connectiion. 
     let (mut socket, _) = listerner.accept().await.unwrap();

     // what we do now is read something from client and write somwthing back
     //read memory from a network stream , we need a buffer to but that in'
     let mut buffer:[u8; 1024] = [0; 1024];

     //use byte read to truncate our buffer when we send to a client

    let byte_read =  socket.read(&mut buffer).await.unwrap();
    socket.write_all(&buffer[..byte_read]).await.unwrap();

    //server has echoed here


}

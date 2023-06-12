use mini_redis::{Command::{self, Get, Set}, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);

    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("Got: {:?}", frame);

    //     let response = Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("{:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => {
                panic!("unimplemented! {:?}", cmd);
            }
        };
    
    connection.write_frame(&response).await.unwrap();
    }
} 
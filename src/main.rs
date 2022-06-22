use tokio::{
    io::{AsyncWriteExt, AsyncBufReadExt, BufReader},
    net::TcpListener, sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (sender, _receiver) = broadcast::channel::<String>(5);
    
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        let sender = sender.clone();
        let mut receiver = sender.subscribe();
        tokio::spawn(async move{
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! { // allow run multiple async statement concurently
                    bytes_read = reader.read_line(&mut line) => {
                        if bytes_read.unwrap() == 0 {
                            break;
                        }

                        sender.send(line.clone()).unwrap();
                        line.clear();
                     }
                     result =  receiver.recv() => {
                        let message = result .unwrap();
                        writer.write_all(message.as_bytes()).await.unwrap();
                     }
                }
            }
        });
    }
}

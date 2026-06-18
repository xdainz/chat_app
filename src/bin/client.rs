use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let connection = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to the server.");

    let (mut tcp_reader, mut tcp_writer) = tokio::io::split(connection);

    let mut stdin_reader = BufReader::new(tokio::io::stdin());
    let mut user_input = String::new();

    let mut server_buffer = [0; 1024];

    'client: loop {
        tokio::select! {
            res = stdin_reader.read_line(&mut user_input)=> {
                let bytes_read = res?;
                if bytes_read == 0 {break;}

                if user_input.trim() == "exit" {
                    println!("Program shutdown.");
                    break 'client;
                }
                tcp_writer.write_all(user_input.as_bytes()).await?;
                user_input.clear();
            }

            res = tcp_reader.read(&mut server_buffer) => {
                let n = res?;

                if n==0 {
                    println!("\nServer closed the connection.");
                    break;
                }

                let response = String::from_utf8_lossy(&server_buffer[..n]);
                print!("{}", response);

                tokio::io::stdout().flush().await?;
            }

        }
    }
    Ok(())
}

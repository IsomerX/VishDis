use bytes::BytesMut;
use clap::{Parser, Subcommand};
use core::result::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
}

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;

    match args.command {
        Command::Get { key } => {
            stream.write_all(b"get").await?;
            stream.write_all(b" ").await?;
            stream.write_all(&key.as_bytes()).await?;

            let mut buf = BytesMut::with_capacity(1024);
            let _length = stream.read_buf(&mut buf).await?;
            match std::str::from_utf8(&mut buf) {
                Ok(resp) => {
                    if resp == "" {
                        println!("Key not found");
                    } else {
                        println!("Value: {}", resp);
                    }
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
            return Ok(());
        }
        Command::Set { key, value } => {
            stream.write_all(b"set").await?;
            stream.write_all(b" ").await?;
            stream.write_all(&key.as_bytes()).await?;
            stream.write_all(b" ").await?;
            stream.write_all(&value.as_bytes()).await?;

            let mut buf = BytesMut::with_capacity(1024);
            let _length = stream.read_buf(&mut buf).await?;
            match std::str::from_utf8(&mut buf) {
                Ok(resp) => {
                    if resp == "r ok" {
                        println!("Key updated successfully");
                    } else if resp == "ok" {
                        println!("Key set successfully");
                    }
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
    }

    Ok(())
}

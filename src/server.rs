use crate::{Handler, Listener};

pub async fn run(listener: &Listener) -> std::io::Result<()> {
    loop {
        let socket = listener.listener.accept().await?.0;
        let mut handler = Handler::new(listener, socket);

        tokio::spawn(async move {
            if let Err(_err) = process_method(&mut handler).await {
                println!("failed to process connection; error = {:?}", _err);
            };
        });
    }
}

async fn process_method(handler: &mut Handler) -> Result<(), std::io::Error> {
    while !handler.shutdown.is_shutdown() {
        let result = tokio::select! {
            _ = handler.shutdown.listen_recv() => {
                return Ok(());
            },
            res = handler.connection.read_buf_data() => res
        };
        let (cmd, vec) = match result {
            Some((cmd, vec)) => (cmd, vec),
            None => return Ok(()),
        };

        handler.process_query(cmd, vec).await?;
    }
    Ok(())
}

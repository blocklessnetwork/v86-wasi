use tokio::sync::mpsc::{channel, Receiver, Sender, error::TryRecvError};

use tokio::{net::{TcpListener, TcpStream}, runtime::Builder};
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use std::time::Duration;
use crate::{bus::BusData, log::LOG};
use std::net::SocketAddr;
use tokio_tungstenite::{accept_async, tungstenite::{Error, Message}};
use tokio_tungstenite::WebSocketStream;

pub(crate) struct WsThread {
    sender: Sender<(Receiver<(u16, BusData)>, Sender<(u16, Vec<u8>)>)>,
}

impl WsThread {
    pub fn new(sender: Sender<(Receiver<(u16, BusData)>, Sender<(u16, Vec<u8>)>)>) -> Self {
        Self {
            sender,
        }
    }

    async fn accept_connection(&mut self, peer: SocketAddr, stream: TcpStream) {
        if let Err(ref e) = self.handle_connection(peer, stream).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => {
                    dbg_log!(LOG::WS, "Error processing connection: {}", err);
                },
            }
        }
    }

    async fn handle_incoming(
        mut sink: SplitSink<WebSocketStream<TcpStream>, Message>,
        mut rx: Receiver<(u16, Vec<u8>)>,
    ) -> Result<(), Error> {
        loop {
            let mut rs = Vec::new();
            let mut msgid;
            let mut data: Vec<u8>;
            loop {
                match rx.try_recv() {
                    Ok((t, d)) => {
                        data = d;
                        msgid = t;
                        let msg_id = msgid.to_le_bytes();
                        let mut msg = Vec::with_capacity(2+data.len());
                        msg.extend(msg_id);
                        msg.extend(data);
                        rs.extend(msg);
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(_) => return Ok(()),
                }
            }
            if rs.len() > 0 {
                let msg = Message::Binary(rs);
                let _ = sink.send(msg).await;
            } else {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        }
    }

    async fn handle_connection(&mut self, _peer: SocketAddr, stream: TcpStream) -> Result<(), Error> {
        let ws_stream = accept_async(stream).await.expect("Failed to accept");
        let (sink, mut ws_stream) = ws_stream.split();
        let (tx, rx) = channel(10240);
        let (s_tx, s_rx) = channel(10240);
        self.sender.send((s_rx, tx)).await.unwrap();
        tokio::spawn(async move {
            Self::handle_incoming(sink, rx).await
        });
        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            match msg {
                Message::Binary(b) => {
                    if b.len() <= 2 {
                        dbg_log!(LOG::WS, "Error msg recv");
                        return Ok(());
                    }
                    let mut msg_id_buf = [0; 2];
                    msg_id_buf.copy_from_slice(&b[..2]);
                    let msg_id = u16::from_le_bytes(msg_id_buf);
                    if msg_id == 0x0100 {
                        if let Err(_) = s_tx.send((msg_id, BusData::U8(b[2]))).await {
                            dbg_log!(LOG::WS, "send Error.");
                            break;
                        }
                    }
                }
                Message::Close(_) => {
                    dbg_log!(LOG::WS, "close by peer.");
                    break;
                },
                _ => {
                    dbg_log!(LOG::WS, "Unknow ws msg recv");
                }
            }
        }
        Ok(())
    }
    

    pub fn start(mut self) {
        let runtime = Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();
        runtime.block_on(async move {
            let listener = TcpListener::bind("127.0.0.1:9002").await.unwrap();
            while let Ok((stream, _)) = listener.accept().await {
                let peer_addr = stream.peer_addr().expect("connected streams should have a peer address");
                self.accept_connection(peer_addr, stream).await;
            }
        });
    }
}
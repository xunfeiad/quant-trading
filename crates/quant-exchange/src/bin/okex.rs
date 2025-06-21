use quant_exchange::okex::{ConnId, OkexWebSocketClient};
use quant_exchange::schema::okex::channel::WebSocketChannelType;
use quant_schema::okex::ws::arg::{WsArg, WsPublicArg, WsPublicInstIdArg};
use tracing_subscriber::EnvFilter;
use tracing::{error, info};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter(
            EnvFilter::new("quant_exchange=trace,okex=trace")
        )
        .init();

    let mut okex_client = OkexWebSocketClient::new(ConnId::default());
    okex_client.0.initialize().await.unwrap();

    let args = vec![WsArg::Public(WsPublicArg::MarkPrice(WsPublicInstIdArg {
        inst_id: "BTC-USD-SWAP".to_string(),
    }))];

    okex_client
        .0
        .subscribe(args.as_slice(), &WebSocketChannelType::Public)
        .await
        .unwrap();

    let receiver_queue_mananger = okex_client.1.public_receiver.clone();

    let task1 = tokio::spawn(async move {
        loop {
            println!("Waiting for messages...");
            let receiver_queue_manager = receiver_queue_mananger.clone();
            match receiver_queue_manager.recv_async().await {
                Ok(message) => {
                    info!("Received message: {:?}", message);
                }
                Err(e) => {
                    error!("Error receiving message: {:?}", e);
                    break;
                }
            }
        }
    });

    let _ = tokio::join!(task1, okex_client.0.start_message_handling());
}

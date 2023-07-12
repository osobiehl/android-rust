use tokio;
use coap_lite::{RequestType as Method};
use coap::Server;
use coap::message::Codec;
use log::{debug, trace, warn, error, info};

pub async fn start_server(){
    let addr = "127.0.0.1:5683";
    info!("Sbefore start server {}", addr);
    let mut server = Server::new(addr).map_err(|e|
        info!("got error: {:?}", e))
        .unwrap();

    // let mut server = Server::new(addr).unwrap();
    info!("Server up on {}", addr);

    server.run(|request| async {
        info!("starting server....");
        match request.get_method() {
            &Method::Get => println!("request by get {}", request.get_path()),
            &Method::Post => println!("request by post {}", String::from_utf8(request.message.payload).unwrap()),
            &Method::Put => println!("request by put {}", String::from_utf8(request.message.payload).unwrap()),
            _ => println!("request by other method"),
        };
        return match request.response {
            Some(mut message) => {
                message.message.payload = b"OK".to_vec();
                Some(message)
            },
            _ => None
        };
    }).await.unwrap();
}
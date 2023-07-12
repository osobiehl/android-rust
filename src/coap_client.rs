use coap_lite::{RequestType as Method, CoapRequest};
use coap::{CoAPClient};
use log::{debug, trace, warn, error, info};

pub fn get_request() {
    let url = "coap://127.0.0.1:5683/Rust";
    debug!("Client request: {}", url);
    let response = CoAPClient::get(url).map_err( |e| info!("error: {:?}" , e)).unwrap();
    debug!("Server reply: {}", String::from_utf8(response.message.payload).unwrap());
}
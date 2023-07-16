#![allow(non_snake_case)]

use std::time::Duration;

use log::{debug, trace, warn, error, info};
mod coap_server;
mod coap_client;
pub mod dtls;
mod keys;
// use tokio::prelude::*;
use tokio::runtime::Runtime;

#[cfg(target_os="android")]
use android_log;
// #[macro_use]
// extern crate log;
// extern crate android_log;

#[no_mangle]
pub unsafe extern "C" fn rust_main(){
    #[cfg(target_os="android")]
    android_log::init("myapp").unwrap();

    dtls::client();
    
    // let rt = Runtime::new().expect("no runtime created");
    // let handle = rt.spawn(coap_server::start_server());
    // coap_client::get_request();
    // rt.block_on(async move {
    //     let  _ = tokio::time::timeout(Duration::from_millis(2000), handle).await;
 
    // });
}
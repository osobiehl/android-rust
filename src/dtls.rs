use std::io::stdin;
use std::net::UdpSocket;
use std::sync::Arc;
use std::time::Duration;

use mbedtls::error::codes;
use mbedtls::rng::CtrDrbg;
use mbedtls::ssl::config::{Endpoint, Preset, Transport};
use mbedtls::ssl::context::Timer;
use mbedtls::ssl::{Config, Context, Io, CookieContext};
use mbedtls::x509::Certificate;
use mbedtls::Result as TlsResult;
use mbedtls::rng::{OsEntropy,EntropyCallback, Random};
use rand;
use log::trace;
use crate::keys;
const SERVER_ADDR: &str = "10.0.2.2";
const SERVER_PORT: &str = "22222";
pub fn client(){
    trace!("starting...");
    let cert = Arc::new(Certificate::from_pem_multiple(keys::ROOT_CA_CERT.as_bytes()).unwrap());
    trace!("got cert");
    let entropy = Arc::new(OsEntropy::new());
    let rng = Arc::new(CtrDrbg::new(entropy, None).unwrap());
    let mut config = Config::new(Endpoint::Client, Transport::Datagram, Preset::Default);
    config.set_rng(rng);
    config.set_ca_list(cert, None);

    let mut ctx = Context::new(Arc::new(config));
    ctx.set_timer_callback(Box::new(mbedtls::ssl::context::Timer::new()));
   
    let sock = UdpSocket::bind("0.0.0.0:12345").unwrap();
    trace!("opened socket");
    let target = format!("{SERVER_ADDR}:{SERVER_PORT}");
    let sock = mbedtls::ssl::io::ConnectedUdpSocket::connect(sock, target).map_err(|e| trace!("error: {:?}", e)).unwrap();
    trace!("try to establish context...");
    ctx.establish(sock, None).unwrap();
   
    trace!("established context!");


    let mut resp: [u8; 200] = [0;200];
    let mut ctr = 1;
    loop{
        trace!("starting...");
        let line = format!("hello, world {ctr}!");
        ctx.send(line.as_bytes()).unwrap();
        let len = ctx.recv(&mut resp).unwrap();
        trace!("receive: {len} bytes!");
        if let Ok(s) = std::str::from_utf8(&resp[..len]) {
            println!("{}", s);
        } else {
            eprintln!("Invalid UTF-8 received");
        }
        std::thread::sleep(Duration::from_millis(500));
        ctr += 1;
    }

}


use mbedtls::pk::Pk;


pub fn server(){
    let entropy = OsEntropy::new();
    let rng: Arc<CtrDrbg> = Arc::new(CtrDrbg::new(Arc::new(entropy), None).unwrap());
    let cert = Arc::new(Certificate::from_pem_multiple(keys::PEM_CERT.as_bytes()).unwrap());
    let cookies = CookieContext::new(rng.clone()).expect("could not create cookies");


    
    let entropy2 = OsEntropy::new();
    let mut rng2  = CtrDrbg::new(Arc::new(entropy2), None).unwrap();
    let key = Arc::new(Pk::from_private_key(&mut rng2 ,keys::PEM_KEY.as_bytes(), None).unwrap());
    let mut config = Config::new(Endpoint::Server, Transport::Datagram, Preset::Default);
    
    config.set_rng(rng);
    config.push_cert(cert, key).unwrap();
    config.set_dtls_cookies(Arc::new(cookies));
    let mut context = Context::new(Arc::new(config));

    


    context.set_timer_callback(Box::new(Timer::new()));
    context.set_client_transport_id_once(b"127.0.0.1:12341");


    let mut sock = UdpSocket::bind(format!("0.0.0.0:{SERVER_PORT}")).unwrap();
    let mut buf: [u8; 2556] = [0;2556];
    let (amount, from) = sock.peek_from(&mut buf).expect("receive nothing!");
    
    println!("got from: {:?}", &from);
    //create a udp socket connected w/
    let sock = mbedtls::ssl::io::ConnectedUdpSocket::connect(sock, from).unwrap();
    

    match context.establish(sock, None) {
        Err(e) if matches!(e.high_level(), Some(codes::SslHelloVerifyRequired)) => {},
        Err(e) => panic!("SslHelloVerifyRequired expected, got {} instead", e),
        Ok(()) => panic!("SslHelloVerifyRequired expected, got Ok instead"),
    }

    context.handshake().expect("could not do handshake");

    
    
    let mut buf: [u8; 2556] = [0;2556];
    loop{
        let r: usize = context.recv(&mut buf).unwrap();
        println!("received from dtls: {}", std::str::from_utf8(&buf[..r]).unwrap());
        println!("sent {} bytes!", context.send( "this is easy".as_bytes()).expect("could not send byte"));
        // context.

    }
}
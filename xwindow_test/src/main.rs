use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

use xwindow::Error;
use xwindow::read_util::WritableWrite;
use xwindow::setup::{ConnectionSetupFailed, read_setup};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();
    let (mut stream, addr) = listener.accept().unwrap();
    println!("{}", addr);
    let mut buffer = [0; 1024];
    match read_setup(&mut stream, &mut buffer) {
        Ok((order, info)) => {
            println!("{:#?}", order);
            println!("{:#?}", info);
            let failed_data = ConnectionSetupFailed {
                protocol_major_version: 11,
                protocol_minor_version: 0,
                reason: String::from("こんにちは！このディスプレイはまだ使えません！ "),
            };
            match stream.write_value(failed_data, &order) {
                Ok(()) => {
                    println!("failed OK!");
                }
                Err(err) => {
                    println!("{:#?}", err);
                }
            };
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    };
    return;
    loop {
        let length = stream.read(&mut buffer).unwrap();
        if length == 0 {
            println!("closed");
            return;
        }
        let mut hex = String::new();
        let mut asc = String::new();
        for c in &buffer[..length] {
            hex.push_str(&format!("{:02x} ", *c));
            match char::try_from(*c as u32) {
                Ok(c) if c.is_ascii() && !c.is_ascii_control() => {
                    asc.push_str(&format!("{}  ", c));
                }
                _ => {
                    asc.push_str(&format!("?  "));
                }
            }
        }
        println!("{}", hex);
        println!("{}", asc);
    }
}
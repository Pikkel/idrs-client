use std::process::Command;
use std::net::UdpSocket;
use std::{thread, time};

fn main() {
    ip();
    let sleep = time::Duration::from_millis(1500);
    thread::sleep(sleep);
    uuid();
}

fn ip() {
    if cfg!(unix) {
        let dick = Command::new("curl").arg("ident.me").output().unwrap();
        let titties = String::from_utf8_lossy(&dick.stdout);
        println!("{}", titties.trim());

        let addr = "127.0.0.1:9999";
        let target = "127.0.0.1:8888";

        let host = UdpSocket::bind(addr).unwrap();

        let mess = titties.trim();

        match host.send_to(mess.as_bytes(), target) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        };
    } else if cfg!(windows) {
        let dick = Command::new("curl").arg("ident.me").output().unwrap();
        let titties = String::from_utf8_lossy(&dick.stdout);
        println!("{}", titties.trim());

        let addr = "127.0.0.1:9999";
        let target = "127.0.0.1:8888";

        let host = UdpSocket::bind(addr).unwrap();

        let mess = titties.trim();

        match host.send_to(mess.as_bytes(), target) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        };
    } else if cfg!(target_os = "macos") {
        let dick = Command::new("curl").arg("ident.me").output().unwrap();
        let titties = String::from_utf8_lossy(&dick.stdout);
        println!("{}", titties.trim());

        let addr = "127.0.0.1:9999";
        let target = "127.0.0.1:8888";

        let host = UdpSocket::bind(addr).unwrap();

        let mess = titties.trim();

        match host.send_to(mess.as_bytes(), target) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        };
    }
}

fn uuid() {
    if cfg!(unix) {
        let balls = Command::new("cat").arg("/etc/machine-id").output().unwrap();
        let titties = String::from_utf8_lossy(&balls.stdout);
        println!("{}", titties.trim());

        let addr = "127.0.0.1:9999";
        let target = "127.0.0.1:8888";

        let host = UdpSocket::bind(addr).unwrap();

        let mess = titties.trim();

        match host.send_to(mess.as_bytes(), target) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        };
    } else if cfg!(windows) {
        let balls = Command::new(
            "reg query HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Cryptography /v MachineGuid",
        )
        .output()
        .unwrap();
        let titties = String::from_utf8_lossy(&balls.stdout);
        println!("{}", titties.trim());

        let addr = "127.0.0.1:9999";
        let target = "127.0.0.1:8888";

        let host = UdpSocket::bind(addr).unwrap();

        let mess = titties.trim();

        match host.send_to(mess.as_bytes(), target) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        };
    } else if cfg!(target_os = "macos") {
        let balls = Command::new("ioreg -rd1 -c IOPlatformExpertDevice | grep IOPlatformUUID")
            .output()
            .unwrap();
        let titties = String::from_utf8_lossy(&balls.stdout);
        println!("{}", titties.trim());

        let addr = "127.0.0.1:9999";
        let target = "127.0.0.1:8888";

        let host = UdpSocket::bind(addr).unwrap();

        let mess = titties.trim();

        match host.send_to(mess.as_bytes(), target) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        };
    }
}

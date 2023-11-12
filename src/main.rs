use std::process::{Command};
use std::thread;
use std::time::Duration;

fn main() {
    let url = "https://example.com";
    let mac_address = "AA:AA:AA:AA:AA:AA"; 
    let sleep_duration = Duration::from_secs(60);



    let mut a = true;
    while a {
        let output = Command::new("wget")
            .arg("-O")
            .arg("-")
            .arg(url)
            .output()
            .expect("err to execute wget");

        if output.status.success() {
            let result_bytes = output.stdout;
            let result_string = String::from_utf8_lossy(&result_bytes);
            println!("{}", result_string);

            if result_string.trim() == "1" {
                send_wake_on_lan_packet(mac_address);
            }
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            eprintln!("Fehler: {}", error_message);
        }
        thread::sleep(sleep_duration);
    }
}
fn send_wake_on_lan_packet(mac_address: &str) {
    let output = Command::new("wakeonlan")
        .arg(mac_address)
        .output()
        .expect("error to execute wakeonlan");

    if output.status.success() {
        println!("Wake-on-LAN sendt to {}", mac_address);
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("error sending wakeonlan: {}", error_message);
    }
}

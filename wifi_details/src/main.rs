use std::process::Command;

fn run(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "(command not found)".to_string())
}

fn main() {
    let ssid = run("nmcli", &["-t", "-f", "active,ssid", "dev", "wifi"]);
    let ssid = ssid
        .lines()
        .find(|l| l.starts_with("yes:"))
        .map(|l| l.trim_start_matches("yes:").to_string())
        .unwrap_or_else(|| "(not connected)".to_string());

    let signal = run("nmcli", &["-f", "IN-USE,SIGNAL", "dev", "wifi"]);
    let signal = signal
        .lines()
        .find(|l| l.starts_with("*"))
        .and_then(|l| l.split_whitespace().nth(1))
        .unwrap_or("unknown");

    let ip_internal = run("hostname", &["-I"]);
    let ip_external = run("curl", &["-s", "https://ifconfig.me"]);

    println!("Wi-Fi Info:");
    println!("  SSID: {ssid}");
    println!("  Signal: {signal}%");
    println!("  Internal IP: {ip_internal}");
    println!("  External IP: {ip_external}");
}

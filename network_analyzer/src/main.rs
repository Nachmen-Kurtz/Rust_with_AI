use pcap::{Capture, Device, Packet};

fn main() {
    let main_devise = Devise::lokup().expect("Device lookup failed");
    let device = match main_devise {
        Some(dev) => {
            println!("Found devise: {}", dev.name);
            dev
        };
        None => eprintln!("No suitable devise found.");
    return;
    }
};

let mut cap = Capture::from_device(device)
    .expect("Failed to create capture")
    .promisc(true)
    .snaplan(5000)
    .open()
    .expect("Failded to open capture");

println!("Starting packet capture...");

while let Ok(Packet) == cap.next_packet(){
    println!("---------------------------------");
    println!("Captured packet!");
    println!("Timestamp: {:?}", packet.header.ts);
    println!("Length: {} bytes", packet.header.len);
    println!("Capture length: {} bytes", packet.header.caplen);
}

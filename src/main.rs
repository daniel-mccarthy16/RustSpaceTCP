use ipv4::Ipv4;
use tcp::Tcp;
pub mod utility;
use tun_tap::{Iface,Mode};
use std::collections::HashMap;
pub mod ipv4;
pub mod tcp;
pub mod connections;
pub mod unixsocket;
use connections::{Connection,SocketPair};
use unixsocket::UnixSocketManager;

fn main()  {

    let iface = Iface::new("mytun", Mode::Tun).expect("Failed to create a TUN device");
    // 
    let mut buffer = [0u8; 1504]; // MTU + 4 for the header
    let mut nbytes: usize;

    let mut connection_table = HashMap::<SocketPair,Connection>::new();
    UnixSocketManager::initialize().expect("[ERROR]: Failed to initialize UnixSocketManager");

    loop { 
        nbytes = iface.recv(&mut buffer).unwrap();
        let _flags = u16::from_be_bytes([buffer[0], buffer[2]]);
        let ethertype = u16::from_be_bytes([buffer[2], buffer[3]]);

        if ethertype != 0x0800 {
            //if not ipv4
            continue;
        }

        match Ipv4::deserialize(&buffer[4..nbytes]) {
            Ok(ipv4header) => {
                println!("Ipv4 header: {:?}", ipv4header);
                if ipv4header.protocol() == 0x06 {
                    //bail here instead of when attempting to read data into a TcpHeaderSLice later
                    //down the line
                    continue;
                }
                let src = ipv4header.source_ip();
                let dst = ipv4header.destination_ip();

                match Tcp::deserialize(&buffer[4+ipv4header.serialize().len()..]) {
                    Ok(tcpheader) => {
                        if !UnixSocketManager::port_is_open(tcpheader.destination_port()) {
                            println!("[INFO]: port blocked {}", tcpheader.destination_port());                            continue;
                        }
                        let mut outbound_packet_buffer = [0u8; 1500];
                        let response_size: usize;
                        let payload_starts_at = (4 + ipv4header.header_length_in_bytes()  + tcpheader.header_length_in_bytes()) as usize;
                        let socket_pair = SocketPair {
                           src_ip : src,
                           dest_ip : dst, 
                           dest_port : tcpheader.destination_port(),
                           src_port : tcpheader.source_port(),
                        };
                        match connection_table.get_mut(&socket_pair) {
                            Some(existing_connection)  => {
                                 match existing_connection.process_incoming(&ipv4header, &tcpheader, &buffer[payload_starts_at..], &mut outbound_packet_buffer) {
                                    Ok(length) => { response_size = length; },
                                    Err(e) => { 
                                        eprintln!("[ERROR]: {}", e);
                                        continue
                                    } 
                                }
                            },
                            None => {
                                if !tcpheader.is_syn_set() {
                                    continue;
                                }
                                println!("New Connection: {}:{} -> {}:{} [SYN:{} ACK:{} FIN:{} RST:{}]",
                                    src, tcpheader.source_port(),
                                    dst, tcpheader.destination_port(),
                                    tcpheader.is_syn_set(), tcpheader.is_ack_set(), tcpheader.is_fin_set(), tcpheader.is_rst_set());
                                match connection_table.insert(socket_pair, Connection::default()) {
                                    Some(mut existing_connection) => {
                                         match existing_connection.process_incoming(&ipv4header, &tcpheader, &buffer[payload_starts_at..], &mut outbound_packet_buffer) {
                                            Ok(length) => { response_size = length; },
                                            Err(e) => { 
                                                eprintln!("[ERROR]: {}", e);
                                                continue
                                            } 
                                        }
                                    }
                                    None => {
                                        eprintln!("[ERROR]: Tried to insert a new connection and didnt get a reference back");
                                        continue;
                                    }
                                }
                            }
                        }

                        if response_size == 0 {
                            println!("[INFO]: no response generated during processing of inbound packet."); 
                            continue;
                        }
                        //write to tun interface
                        match iface.send(&outbound_packet_buffer[..response_size]) {
                                Ok(bytecount) => {
                                    println!("[INFO]: Successfully wrote {} bytes to the tunnel interface", bytecount);
                                }
                                Err(e) => {
                                    eprintln!("[ERROR]: writing to tunnel interface: {}", e);
                                }
                        }
                    }
                    Err(_value) => continue,
                }
            
            },
            Err(value) => {
                println!("Err {:?}", value)
            }
        }

    }
}

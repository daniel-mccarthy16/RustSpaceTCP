use std::net::Ipv4Addr;
use std::default::Default;
use crate::Ipv4;
use crate::Tcp;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct SocketPair {
    pub src_ip: Ipv4Addr,
    pub dest_ip: Ipv4Addr,
    pub src_port: u16, 
    pub dest_port: u16
}


pub struct Connection {
    connection_state: ConnectionState,
    client_sequence_number: u32,
    server_sequence_number: u32,
    client_acknowledgement_number: u32,
    server_acknowledgement_number: u32,
    client_window: u16,
    server_window: u16,
    inbound_buffer: Vec<u8>,
    outbound_buffer: Vec<u8>
}



impl Default for Connection {
    fn default() -> Self {
        Connection {
            client_sequence_number : 0,
            server_sequence_number : 0,
            connection_state : ConnectionState::Unitialized,
            client_acknowledgement_number : 0,
            server_acknowledgement_number : 0,
            server_window: 65535,
            client_window: 0,
            inbound_buffer: Vec::new(),
            outbound_buffer: Vec::new()
        }
    }
}


impl Connection {

    pub fn process_incoming(&mut self, incoming_ipv4header: &Ipv4, incoming_tcpheader: &Tcp, payload: &[u8], outbound_buffer: &mut [u8]) -> Result<usize, String> {
        
        self.client_window = incoming_tcpheader.window_size();

        match self.connection_state {
            ConnectionState::Unitialized => {
                self.client_sequence_number = incoming_tcpheader.sequence_number();
                self.client_window = incoming_tcpheader.window_size();
                self.server_acknowledgement_number = self.client_sequence_number + 1;
                self.server_sequence_number = Self::set_intial_sequence_number();
                let mut outbound_tcp_header = Tcp::default();
                let mut outbound_ipv4_header = Ipv4::new(
                   incoming_ipv4header.destination_ip(),
                   incoming_ipv4header.source_ip(),
                );
                outbound_tcp_header.set_syn_ack_flags();
                outbound_tcp_header.set_window(self.server_window);
                outbound_tcp_header.set_acknowledgement_number(self.server_acknowledgement_number);
                outbound_tcp_header.set_sequence_number(self.server_sequence_number);
                let mut serialized_tcp_header = outbound_tcp_header.serialize();
                let pseduo_header = Tcp::create_checksum_pseudo_header(
                    incoming_ipv4header.destination_ip(),
                    incoming_ipv4header.source_ip(),
                    &serialized_tcp_header,
                    payload
                );
                let tcp_checksum = Tcp::calculate_tcp_checksum(pseduo_header.as_slice(), &serialized_tcp_header, &payload);
                Tcp::write_checksum(&mut serialized_tcp_header, tcp_checksum);
                outbound_ipv4_header.set_total_length(20 as u16 + serialized_tcp_header.len() as u16);

                //serialize ipv4 header
                let mut serialized_ipv4_header = outbound_ipv4_header.serialize();
                Ipv4::calculate_and_set_checksum(&mut serialized_ipv4_header);

                let ipv4_header_length = serialized_ipv4_header.len();
                let tcp_header_length = serialized_tcp_header.len();
                outbound_buffer[ .. ipv4_header_length ].copy_from_slice(serialized_ipv4_header.as_slice());  
                outbound_buffer[ ipv4_header_length .. ipv4_header_length + tcp_header_length  ].copy_from_slice(serialized_tcp_header.as_slice());  

                self.connection_state = ConnectionState::SynReceived;
                Ok(outbound_ipv4_header.total_length() as usize)

            }
            ConnectionState::Listen => todo!(),
            ConnectionState::SynReceived => {
                //NO RESPONSE NEEDED HERE
                if !incoming_tcpheader.is_ack_set()  {
                    return Err("[ERROR]: expected cliient to acknowledge our servers syn flag".to_string());
                }

                if !incoming_tcpheader.acknowledgement_number() == self.server_sequence_number + 1 {
                    return Err("[ERROR]: expected client to acknowledge our server ISN + 1".to_string());
                }

                if !incoming_tcpheader.sequence_number() == self.server_acknowledgement_number {
                    return Err("[ERROR]: Client did not update sequence number to matched what we acknowledged in our synack response".to_string());
                }

                self.client_sequence_number = incoming_tcpheader.sequence_number();
                self.client_window = incoming_tcpheader.window_size();
                self.server_acknowledgement_number = self.client_sequence_number + 1;
                self.server_sequence_number = self.server_sequence_number + 1;

                self.connection_state = ConnectionState::Established;
                println!("[INFO] successfully established tcp connection");

                Ok(0 as usize)

            },
            ConnectionState::SynSent => todo!(),
            ConnectionState::Established => {
                println!("Wtf is going on here...");
                return Ok(0 as usize)
            },
            ConnectionState::FinWait1 => todo!(),
            ConnectionState::FinWait2 => todo!(),
            ConnectionState::CloseWait => todo!(),
            ConnectionState::Closing => todo!(),
            ConnectionState::LastAct => todo!(),
            ConnectionState::TimeWait => todo!(),
            ConnectionState::Closed => todo!(),
        }
        

    }
   pub fn set_intial_sequence_number () -> u32 {
        1
   }

     

}


enum ConnectionState {
    Unitialized,
    Listen,
    SynReceived,
    SynSent,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAct,
    TimeWait,
    Closed
}

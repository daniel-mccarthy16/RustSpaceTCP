use crate::utility::calculate_checksum;
use std::net::Ipv4Addr;

#[derive(Debug)]
#[repr(C, packed)]
pub struct Tcp {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgement_number: u32,
    data_offset_and_reserved: u8, // Combined field for data offset (4 bits) and reserved (4 bits)
    flags: u8, // TCP flags (9 bits, but typically represented in a byte for simplicity)
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16,
    // Options would be here if needed, variable length
}


impl Default for Tcp {
    fn default() -> Self {
        Tcp {
            source_port: 0,
            destination_port: 0,
            sequence_number: 0,
            acknowledgement_number: 0,
            data_offset_and_reserved: 0x50, // Already correctly set
            flags: 0,
            window_size: 0,
            checksum: 0,
            urgent_pointer: 0,
            // Initialize other fields as needed
        }
    }
}

impl Tcp {

    pub fn new(
        source_port: u16,
        destination_port: u16,
        sequence_number: u32,
        acknowledgement_number: u32,
        data_offset_and_reserved: u8, // Combined field for data offset (4 bits) and reserved (4 bits)
        flags: u8, // TCP flags (9 bits, but typically represented in a byte for simplicity)
        window_size: u16,
        checksum: u16,
        urgent_pointer: u16,

    ) -> Tcp {
        Tcp {
            source_port,
            destination_port,
            sequence_number,
            acknowledgement_number,
            data_offset_and_reserved, // Combined field for data offset (4 bits) and reserved (4 bits)
            flags, // TCP flags (9 bits, but typically represented in a byte for simplicity)
            window_size,
            checksum,
            urgent_pointer,
        }
    }

    pub fn deserialize(data: &[u8]) -> Result<Tcp, &'static str> {

        if data.len() < 20 {
            return Err("Data too short for TCP header");
        }
        
        let source_port = (( data[0] as u16 )  << 8) | data[1] as u16;
        let destination_port = (( data[2] as u16 )  << 8) | data[3] as u16;
        let sequence_number = ((data[4] as u32) << 24)
                                |  ((data[5] as u32) << 16) 
                                | ((data[6] as u32) << 8) 
                                | (data[7] as u32); 
        let acknowledgement_number = ((data[8] as u32) << 24)
                                |  ((data[9] as u32) << 16) 
                                | ((data[10] as u32) << 8) 
                                | (data[11] as u32); 
        let data_offset_and_reserved = data[12];
        let flags = data[13];
        let window_size = (( data[14] as u16 )  << 8 ) | data[15] as u16;
        let checksum = (( data[16] as u16 )  << 8 ) | data[17] as u16;
        let urgent_pointer = (( data[18] as u16 )  << 8 ) | data[19] as u16;

        return Ok(Tcp {
            source_port,
            destination_port,
            sequence_number,
            acknowledgement_number,
            data_offset_and_reserved,
            flags,
            window_size,
            checksum,
            urgent_pointer
        })

    }

   pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(20);  // TCP header is typically 20 bytes without options
        buffer.extend_from_slice(&(self.source_port.to_be_bytes()));
        buffer.extend_from_slice(&(self.destination_port.to_be_bytes()));
        buffer.extend_from_slice(&(self.sequence_number.to_be_bytes()));
        buffer.extend_from_slice(&(self.acknowledgement_number.to_be_bytes()));
        buffer.push(self.data_offset_and_reserved);
        buffer.push(self.flags);
        buffer.extend_from_slice(&(self.window_size.to_be_bytes()));
        buffer.extend_from_slice(&(self.checksum.to_be_bytes()));
        buffer.extend_from_slice(&(self.urgent_pointer.to_be_bytes()));
        buffer
    }

    pub fn header_length_in_bytes(&self) -> u8 {
        (self.data_offset_and_reserved >> 4) * 4
    }

    pub fn destination_port(&self) -> u16 {
        self.destination_port
    }

    pub fn source_port(&self) -> u16 {
        self.source_port
    }

    pub fn window_size(&self) -> u16 {
        self.window_size
    }

    pub fn sequence_number(&self) -> u32 {
        self.sequence_number
    }

    pub fn set_sequence_number(&mut self, number: u32) {
        self.sequence_number = number;
    }

    pub fn acknowledgement_number(&self) -> u32 {
        self.acknowledgement_number
    }

    pub fn set_acknowledgement_number(&mut self, number: u32) {
        self.acknowledgement_number = number;
    }

    pub fn is_urg_set(&self) -> bool {
        self.flags & 0b00100000 != 0
    }

    pub fn is_ack_set(&self) -> bool {
        self.flags & 0b00010000 != 0
    }

    pub fn is_psh_set(&self) -> bool {
        self.flags & 0b00001000 != 0
    }

    pub fn is_rst_set(&self) -> bool {
        self.flags & 0b00000100 != 0
    }

    pub fn is_syn_set(&self) -> bool {
        self.flags & 0b00000010 != 0
    }

    pub fn is_fin_set(&self) -> bool {
        self.flags & 0b00000001 != 0
    }


    pub fn set_syn_ack_flags(&mut self) {
        self.flags = 0b0001_0010;
    }

    pub fn set_window(&mut self, window_size: u16) {
        self.window_size = window_size;
    }


    pub fn calculate_tcp_checksum(pseudoheader: &[u8], tcpheader: &[u8], payload: &[u8]) -> u16 {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(pseudoheader);
        buffer.extend_from_slice(tcpheader);
        buffer.extend_from_slice(payload);
        calculate_checksum(&buffer) 
    }

    pub fn create_checksum_pseudo_header(
        source_ip: Ipv4Addr,
        destination_ip: Ipv4Addr,
        serialized_header: &[u8],
        payload: &[u8]
    ) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&source_ip.octets());
        buffer.extend_from_slice(&destination_ip.octets());
        buffer.push(0); //0x00 byte
        buffer.push(6); //6 represents TCP4 
      // Calculate and push TCP length (header + payload)
        let tcp_length = (serialized_header.len() + payload.len()) as u16;
        buffer.extend_from_slice(&tcp_length.to_be_bytes()); // Convert to big endian and add

        buffer

    }

    pub fn write_checksum(tcpheader: &mut Vec<u8>, checksum: u16) {
       tcpheader[16] = (checksum >> 8) as u8;
       tcpheader[17] = checksum as u8;
    }


}



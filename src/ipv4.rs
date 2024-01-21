use std::net::Ipv4Addr;
use crate::utility::calculate_checksum;

//stops compiler rearanging/optimzing bits in this structure.
//This is no bueno as we ultimately want to serialize this and foward it over the network
#[derive(Debug)]
#[repr(C, packed)]
pub struct Ipv4 {
    version_and_ihl: u8,
    type_of_service: u8,
    total_length: u16,
    identification: u16,
    flags_and_fragment_offset: u16,
    ttl: u8,
    protocol: u8,
    header_checksum: u16,
    source_address: Ipv4Addr,
    destination_address: Ipv4Addr
}

impl Ipv4 {
     pub fn new(
            source_address: Ipv4Addr,
            destination_address: Ipv4Addr,
        ) -> Ipv4 {
            Ipv4 {
                version_and_ihl : 0x45,
                type_of_service : 0x00,
                total_length: 20,
                identification: 0,
                flags_and_fragment_offset : 0x0000,
                ttl: 64,
                protocol : 0x06, //TCP
                header_checksum : 0 ,
                source_address,
                destination_address
            }
        }

    pub fn deserialize(data: &[u8]) -> Result<Ipv4, &'static str> {

        println!("[NOTICE]: attempting to deserialize IPV4 header");

        if data.len() < 20 {
            return Err("[ERROR]: Not enough bytes to constitute a valid Ipv4 header");
        }

        let version_and_ihl = data[0];

        if version_and_ihl >> 4 != 4 {
            return Err("[ERROR]: Packet is not declaring itself as a version Ipv4 packet");
        }


        let type_of_service = data[1];
        let total_length = ((data[2] as u16) << 8) | data[3] as u16;
        let identification = ((data[4] as u16) << 8) | data[5] as u16;
        let flags_and_fragment_offset = ((data[6] as u16) << 8) | data[7] as u16;
        let ttl = data[8];
        let protocol = data[9];
        let header_checksum = ((data[10] as u16) << 8) | data[11] as u16;
        let source_address = Ipv4Addr::new(data[12], data[13], data[14], data[15]);
        let destination_address = Ipv4Addr::new(data[16], data[17], data[18], data[19]);

        let header_length = ( version_and_ihl & 0x0F ) as usize * 4;

        let calculated_checksum = calculate_checksum(&data[..header_length]);
        if calculated_checksum != header_checksum {
            return Err("[ERROR]: checksum didnt match");
        } else {
            println!("[NOTICE]: checksum matches!!");
        }

        Ok(Ipv4 {
            version_and_ihl,
            type_of_service,
            total_length,
            identification,
            flags_and_fragment_offset,
            ttl,
            protocol,
            header_checksum,
            source_address,
            destination_address
        })
    }

    pub fn serialize(&self) -> Vec<u8> {

        let mut bytes = Vec::new();
        bytes.push(self.version_and_ihl);
        bytes.push(self.type_of_service);

        bytes.push((self.total_length >> 8) as u8);
        bytes.push(self.total_length as u8);

        bytes.push((self.identification >> 8) as u8);
        bytes.push(self.identification as u8);

        bytes.push((self.flags_and_fragment_offset >> 8) as u8);
        bytes.push(self.flags_and_fragment_offset as u8);

        bytes.push(self.ttl);
        bytes.push(self.protocol);

        //space for 2 byte checksum
        bytes.push(0u8);
        bytes.push(0u8);

        bytes.extend_from_slice(&self.source_address.octets());
        bytes.extend_from_slice(&self.destination_address.octets());

        bytes

    }

    pub fn calculate_and_set_checksum ( serialized_packet: &mut Vec<u8> ) {
        let checksum = calculate_checksum(serialized_packet.as_slice());
        serialized_packet[11] = ( checksum  >> 8 )as u8;
        serialized_packet[12] = checksum as u8;
    }

    pub fn protocol(&self) -> u8 {
        self.protocol
    }

    pub fn source_ip(&self) -> Ipv4Addr {
        self.source_address
    }

    pub fn destination_ip(&self) -> Ipv4Addr {
        self.destination_address
    }

    pub fn total_length(&self) -> u16 {
        self.total_length
    }

    pub fn set_total_length(&mut self, length: u16) {
        self.total_length = length
    }

    pub fn header_length_in_bytes(&self) -> u8 {
        (self.version_and_ihl & 0x0F) * 4
    }

}

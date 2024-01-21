pub fn calculate_checksum(header: &[u8]) -> u16 {

    let mut sum = 0u32;

    for (i, &byte) in header.iter().enumerate().step_by(2) {

        if  i == 10  {
            continue;
        }

        let mut temp2bytes = (byte as u16) << 8;
        if i + 1 < header.len() {
            temp2bytes = temp2bytes | (header[i+1] as u16);
        }
        sum = sum.wrapping_add(temp2bytes as u32);
    };

    //we have overflow bytes
    while sum >> 16 != 0 {
        //and bitwise operator is grabbing lowest 16 bits for us
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    //flip all bits aka return the 1's compliment
    !(sum as u16)

}



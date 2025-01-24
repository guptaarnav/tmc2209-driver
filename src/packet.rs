//! Packet building (read/write) and CRC calculation for the TMC2209.
//! This module is `no_std` friendly, just manipulating bytes.

/// Calculate the 8-bit CRC for TMC2209 packets.
/// Polynomial is x^8 + x^2 + x + 1, LSB-first.
pub fn calc_crc8(bytes: &[u8]) -> u8 {
    let mut crc: u8 = 0;
    for &b in bytes {
        let mut current = b;
        for _ in 0..8 {
            let mix = (crc ^ current) & 0x01;
            crc >>= 1;
            if mix != 0 {
                // 0x8C = 0b10001100 => reversed polynomial for LSB-first
                crc ^= 0x8C;
            }
            current >>= 1;
        }
    }
    crc
}

/// Build an 8-byte write packet for a 32-bit register write.
///
/// Layout: [addrByte, regByte, data0, data1, data2, data3, crc, 0]
pub fn build_write_packet(slave: u8, reg_addr: u8, value: u32) -> [u8; 8] {
    // Address byte: 0x05 in upper nibble, plus 4 bits for slave
    let adr_byte = (0x05 << 4) | (slave & 0x0F);

    // For a write, the register's top bit (bit7) must be 0
    let reg_byte = reg_addr & 0x7F;

    let d0 = (value & 0xFF) as u8;
    let d1 = ((value >> 8) & 0xFF) as u8;
    let d2 = ((value >> 16) & 0xFF) as u8;
    let d3 = ((value >> 24) & 0xFF) as u8;

    let mut packet = [0u8; 8];
    packet[0] = adr_byte;
    packet[1] = reg_byte;
    packet[2] = d0;
    packet[3] = d1;
    packet[4] = d2;
    packet[5] = d3;
    // Byte 6 => CRC
    packet[6] = calc_crc8(&packet[..6]);
    // Byte 7 => not used, can be 0
    packet
}

/// Build a 4-byte read packet to request data from a TMC2209 register.
///
/// Layout: [addrByte, regByte|0x80, crc, 0]
pub fn build_read_packet(slave: u8, reg_addr: u8) -> [u8; 4] {
    let adr_byte = (0x05 << 4) | (slave & 0x0F);
    // For a read, bit7 = 1
    let reg_byte = (reg_addr & 0x7F) | 0x80;

    let mut packet = [0u8; 4];
    packet[0] = adr_byte;
    packet[1] = reg_byte;
    // CRC covers bytes 0..1
    packet[2] = calc_crc8(&packet[..2]);
    // Byte 3 => not used, can be 0
    packet
}

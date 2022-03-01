use crate::buffer::Buffer;
use crate::packet::OperatorLenType::*;

pub enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

pub fn parse_packet(buffer: &mut Buffer) -> (Packet, u64) {
    let version = buffer.read_bits(3).unwrap();
    let type_id = buffer.read_bits(3).unwrap();
    let (packet, mut read) = match type_id {
        4 => parse_literal_packet(version, buffer),
        type_id => parse_operator_packet(version, type_id, buffer),
    };
    read += 6;

    (packet, read)
}

pub struct LiteralPacket {
    pub version: u8,
    pub value: u64,
}

fn parse_literal_packet(version: u8, buffer: &mut Buffer) -> (Packet, u64) {
    let mut value = 0_u64;
    let mut read = 0_u64;
    loop {
        let quintet = buffer.read_bits(5).unwrap();
        let val_bits = quintet & 0b01111;
        let end_bits = quintet & 0b10000;
        value = (value << 4) | (val_bits as u64);
        read += 5;
        if end_bits == 0 {
            break;
        }
    }

    (Packet::Literal(LiteralPacket { version, value }), read)
}

pub struct OperatorPacket {
    pub version: u8,
    pub type_id: u8,
    pub len_type: OperatorLenType,
    pub operation: Operation,
    pub sub_packets: Vec<Packet>,
}

pub enum OperatorLenType {
    Length(u64),
    Quantity(u64),
}

impl OperatorLenType {
    const LENGTH: u8 = 0;
    const QUANTITY: u8 = 1;
}

pub enum Operation {
    Sum,
    Prod,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl Operation {
    fn from_num(n: u8) -> Self {
        use Operation::*;
        match n {
            0 => Sum,
            1 => Prod,
            2 => Min,
            3 => Max,
            5 => Gt,
            6 => Lt,
            7 => Eq,
            _ => panic!("wrong"),
        }
    }
}

fn parse_operator_packet(version: u8, type_id: u8, buffer: &mut Buffer) -> (Packet, u64) {
    let mut read = 0_u64;

    let len_type = match buffer.read_bits(1).unwrap() {
        OperatorLenType::LENGTH => {
            let first = buffer.read_bits(7).unwrap();
            let second = buffer.read_bits(8).unwrap();
            read += 16;
            Length(((first as u64) << 8) | (second as u64))
        }
        OperatorLenType::QUANTITY => {
            let first = buffer.read_bits(3).unwrap();
            let second = buffer.read_bits(8).unwrap();
            read += 12;
            Quantity(((first as u64) << 8) | (second as u64))
        }
        _ => unreachable!(),
    };

    let (sub_packets, sub_read) = match &len_type {
        Length(len) => parse_sub_packets_from_len(buffer, *len),
        Quantity(qua) => parse_sub_packets_from_quantity(buffer, *qua),
    };
    read += sub_read;

    (
        Packet::Operator(OperatorPacket {
            version,
            type_id,
            len_type,
            operation: Operation::from_num(type_id),
            sub_packets,
        }),
        read,
    )
}

fn parse_sub_packets_from_len(buffer: &mut Buffer, len: u64) -> (Vec<Packet>, u64) {
    let mut sub_packets = Vec::with_capacity(3);
    let mut read = 0_u64;
    loop {
        let (packet, r) = parse_packet(buffer);
        sub_packets.push(packet);
        read += r;
        if read >= len {
            break;
        }
    }
    (sub_packets, read)
}

fn parse_sub_packets_from_quantity(buffer: &mut Buffer, quantity: u64) -> (Vec<Packet>, u64) {
    let mut sub_packets = Vec::with_capacity(3);
    let mut read = 0_u64;
    for _ in 0..quantity {
        let (sub_packet, r) = parse_packet(buffer);
        sub_packets.push(sub_packet);
        read += r;
    }
    (sub_packets, read)
}

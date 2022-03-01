use super::packet::{Operation, Packet};
use std::ops::*;

pub fn evaluate_packet(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(lt) => lt.value,
        Packet::Operator(op) => match &op.operation {
            Operation::Sum => sum_packets(&op.sub_packets),
            Operation::Prod => prod_packets(&op.sub_packets),
            Operation::Min => min_packets(&op.sub_packets),
            Operation::Max => max_packets(&op.sub_packets),
            Operation::Gt => greater_than_packets(&op.sub_packets),
            Operation::Lt => less_than_packets(&op.sub_packets),
            Operation::Eq => equal_packets(&op.sub_packets),
        },
    }
}

fn sum_packets(packets: &[Packet]) -> u64 {
    packets.iter().map(evaluate_packet).fold(0, u64::add)
}

fn prod_packets(packets: &[Packet]) -> u64 {
    packets.iter().map(evaluate_packet).fold(1, u64::mul)
}

fn min_packets(packets: &[Packet]) -> u64 {
    packets.iter().map(evaluate_packet).min().unwrap()
}

fn max_packets(packets: &[Packet]) -> u64 {
    packets.iter().map(evaluate_packet).max().unwrap()
}

fn greater_than_packets(packets: &[Packet]) -> u64 {
    if packets.len() != 2 {
        panic!("wrong");
    }
    let first = evaluate_packet(&packets[0]);
    let second = evaluate_packet(&packets[1]);
    if first > second {
        1
    } else {
        0
    }
}

fn less_than_packets(packets: &[Packet]) -> u64 {
    if packets.len() != 2 {
        panic!("wrong");
    }
    let first = evaluate_packet(&packets[0]);
    let second = evaluate_packet(&packets[1]);
    if second > first {
        1
    } else {
        0
    }
}

fn equal_packets(packets: &[Packet]) -> u64 {
    if packets.len() != 2 {
        panic!("wrong");
    }
    let first = evaluate_packet(&packets[0]);
    let second = evaluate_packet(&packets[1]);
    if first == second {
        1
    } else {
        0
    }
}

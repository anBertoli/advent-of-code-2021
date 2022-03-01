use day_16::buffer::*;
use day_16::packet::*;
use std::fs;
use std::ops::Add;

fn main() {
    println!("Day 16!");
    let mut input_buffer = read_input("input.txt");
    let (packet, _) = parse_packet(&mut input_buffer);

    // Part 1.
    let versions_sum = sum_versions(&packet);
    println!("Part 1. Versions sum: {:?}", versions_sum);

    // Part 2.
    let value = evaluate_packet(&packet);
    println!("Part 2. Evaluated packet: {}", value);
}

fn sum_versions(packet: &Packet) -> u32 {
    match packet {
        Packet::Literal(lt_packet) => lt_packet.version as u32,
        Packet::Operator(op_packet) => {
            op_packet.version as u32
                + op_packet
                    .sub_packets
                    .iter()
                    .map(sum_versions)
                    .fold(0, u32::add)
        }
    }
}

fn read_input(path: &str) -> Buffer {
    let file_contents = fs::read_to_string(path).unwrap();
    Buffer::from_hex_string(&file_contents)
}

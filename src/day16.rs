/// A struct representing a hexadecimal value.
/// If least significant four bytes are 0,
/// it has been depleted.
/// ABCD_0001 -> A
/// BCD0_0010 -> B
/// CD00_0100 -> C
/// D000_1000 -> D
/// 0001_0000 -> empty!
struct Hex(u8);

impl Hex {
    fn from_hex_u8(hex: u8) -> Self {
        Hex(0b0000_0001
            | (match hex {
                b'0'..=b'9' => hex - b'0',
                b'A'..=b'F' => hex + 10 - b'A',
                _ => panic!("invalid hex: {}", hex),
            } << 4))
    }

    fn empty() -> Self {
        Hex(0)
    }

    fn is_at_start(&self) -> bool {
        self.0 & 0b1 == 1
    }
}

impl Iterator for Hex {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if 0b0000_1111 & self.0 == 0 {
            None
        } else {
            let val = 0b1000_0000 & self.0 != 0;
            self.0 <<= 1;
            Some(val)
        }
    }
}

struct SliceHexReader<T>
where
    T: Iterator<Item = u8>,
{
    current: Hex,
    remainder: T,
    /// How many bits have been read (or None's!)
    read: usize,
}

impl<T> SliceHexReader<T>
where
    T: Iterator<Item = u8>,
{
    fn skip_hex_remainder(&mut self) {
        if self.current.is_at_start() {
            self.load_next_hex();
        }
    }

    fn load_next_hex(&mut self) {
        self.current = self
            .remainder
            .next()
            .map(Hex::from_hex_u8)
            .unwrap_or(Hex::empty());
    }

    fn new(remainder: T) -> Self {
        SliceHexReader {
            current: Hex::empty(),
            remainder,
            read: 0,
        }
    }
}

impl<T> Iterator for SliceHexReader<T>
where
    T: Iterator<Item = u8>,
{
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        self.read += 1;
        if let Some(bit) = self.current.next() {
            Some(bit)
        } else {
            self.load_next_hex();
            self.current.next()
        }
    }
}

/// Read a number
fn number<T, Out>(input: &mut SliceHexReader<T>, len: usize) -> Out
where
    T: Iterator<Item = u8>,
    Out: num::Zero + num::One + std::ops::Shl<Output = Out> + std::ops::BitOr<Output = Out>,
{
    input.take(len).fold(Out::zero(), |acc, x| {
        acc << Out::one() | if x { Out::one() } else { Out::zero() }
    })
}

fn value_block<T>(input: &mut SliceHexReader<T>) -> (bool, u8)
where
    T: Iterator<Item = u8>,
{
    let next = input.next().unwrap();
    let value = number(input, 4);
    (next, value)
}

fn value<T>(input: &mut SliceHexReader<T>) -> usize
where
    T: Iterator<Item = u8>,
{
    let mut value = 0;
    loop {
        let (cont, val_block) = value_block(input);
        value <<= 4;
        value |= usize::from(val_block);
        if !cont {
            return value;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PacketHeader {
    packet_version: u8,
    packet_type_id: u8,
}

impl PacketHeader {
    fn read<T>(input: &mut SliceHexReader<T>) -> Self
    where
        T: Iterator<Item = u8>,
    {
        let packet_version = number(input, 3);
        let packet_type_id = number(input, 3);
        PacketHeader {
            packet_version,
            packet_type_id,
        }
    }
}

struct Packet {
    version_number_sum: usize,
    value: usize
}

trait PacketOperator {
    fn first_value() -> usize;
    fn op(val_1: usize, val_2: usize) -> usize;
}

struct SumOp;

impl PacketOperator for SumOp {
    fn first_value() -> usize {
        0
    }

    fn op(val_1: usize, val_2: usize) -> usize {
        val_1 + val_2
    }
}

struct ProductOp;

impl PacketOperator for ProductOp {
    fn first_value() -> usize {
        1
    }

    fn op(val_1: usize, val_2: usize) -> usize {
        val_1 * val_2
    }
}


struct MinOp;

impl PacketOperator for MinOp {
    fn first_value() -> usize {
        usize::MAX
    }

    fn op(val_1: usize, val_2: usize) -> usize {
        val_1.min(val_2)
    }
}


struct MaxOp;

impl PacketOperator for MaxOp {
    fn first_value() -> usize {
        usize::MIN
    }

    fn op(val_1: usize, val_2: usize) -> usize {
        val_1.max(val_2)
    }
}

struct GreaterThanOp;
impl PacketOperator for GreaterThanOp {
    fn first_value() -> usize {
        usize::MAX
    }

    fn op(val_1: usize, val_2: usize) -> usize {
        if val_1 == Self::first_value() {
            val_2
        } else if val_1 > val_2 {
            1
        } else {
            0
        }
    }
}


struct LessThanOp;
impl PacketOperator for LessThanOp {
    fn first_value() -> usize {
        usize::MAX
    }

    fn op(val_1: usize, val_2: usize) -> usize {
        if val_1 == Self::first_value() {
            val_2
        } else if val_1 < val_2 {
            1
        } else {
            0
        }
    }
}

struct EqualOp;
impl PacketOperator for EqualOp {
    fn first_value() -> usize {
        usize::MAX
    }

    fn op(val_1: usize, val_2: usize) -> usize {
        if val_1 == Self::first_value() {
            val_2
        } else if val_1 == val_2 {
            1
        } else {
            0
        }
    }
}

impl Packet {
    fn read<T>(input: &mut SliceHexReader<T>) -> Packet
    where
        T: Iterator<Item = u8>,
    {
        let header = PacketHeader::read(input);
        match header.packet_type_id {
            0 => Packet::read_operator_packet::<_, SumOp>(input, header),
            1 => Packet::read_operator_packet::<_, ProductOp>(input, header),
            2 => Packet::read_operator_packet::<_, MinOp>(input, header),
            3 => Packet::read_operator_packet::<_, MaxOp>(input, header),
            4 => Packet::read_literal_value_packet(input, header),
            5 => Packet::read_operator_packet::<_, GreaterThanOp>(input, header),
            6 => Packet::read_operator_packet::<_, LessThanOp>(input, header),
            7 => Packet::read_operator_packet::<_, EqualOp>(input, header),
            _ => unreachable!()
        }
    }

    fn read_literal_value_packet<T>(input: &mut SliceHexReader<T>, header: PacketHeader) -> Packet
    where
        T: Iterator<Item = u8>,
    {
        let value = value(input);
        input.skip_hex_remainder();
        let version_number_sum = usize::from(header.packet_version);
        Packet {
            version_number_sum,
            value
        }
    }

    fn read_operator_packet<T, Op: PacketOperator>(input: &mut SliceHexReader<T>, header: PacketHeader) -> Packet
    where
        T: Iterator<Item = u8>,
    {
        let length_type_id = input.next().unwrap();

        let inital_fold_value = (usize::from(header.packet_version), Op::first_value());
        let fold_fn = |(version_number_sum, value), packet: Packet| {
            (version_number_sum + packet.version_number_sum, Op::op(value, packet.value))
        };

        let (version_number_sum, value) = if length_type_id {
            let number_of_sub_packets: usize = number(input, 11);
            (0..number_of_sub_packets)
                .map(|_| Packet::read(input))
                .fold(inital_fold_value, fold_fn)
        } else {
            let total_bit_length = number::<_, usize>(input, 15) + input.read;
            std::iter::from_fn(|| {
                if input.read < total_bit_length {
                    Some(Packet::read(input))
                } else {
                    None
                }
            })
            .fold(inital_fold_value, fold_fn)
        };
        input.skip_hex_remainder();
        Packet {
            version_number_sum,
            value
        }
    }

    fn value(&self) -> usize {
        unimplemented!()
    }
}

pub fn part_1(input: &str) -> usize {
    let mut bits = SliceHexReader::new(input.bytes());
    let packet = Packet::read(&mut bits);
    packet.version_number_sum
}

pub fn part_2(input: &str) -> usize {
    let mut bits = SliceHexReader::new(input.bytes());
    let packet = Packet::read(&mut bits);
    packet.value
}

#[test]
fn test_bit_decoding() {
    let fold_to_str = |mut s: String, x: bool| {
        if x {
            s.push('1');
        } else {
            s.push('0');
        }
        s
    };

    assert_eq!(SliceHexReader::new("D2FE28".bytes()).fold(String::new(), fold_to_str), "110100101111111000101000");
    assert_eq!(SliceHexReader::new("38006F45291200".bytes()).fold(String::new(), fold_to_str), "00111000000000000110111101000101001010010001001000000000");
    assert_eq!(SliceHexReader::new("EE00D40C823060".bytes()).fold(String::new(), fold_to_str), "11101110000000001101010000001100100000100011000001100000");
}

#[test]
fn read_packet_header_test() {
    let packet = "D2FE28";
    let mut reader = SliceHexReader::new(packet.bytes());
    let header = PacketHeader::read(&mut reader);
    assert_eq!(header, PacketHeader {
        packet_version: 6,
        packet_type_id: 4
    });
}

#[test]
fn test_part_1_example_1() {
    assert_eq!(part_1("8A004A801A8002F478"), 16);
}

#[test]
fn test_part_1_example_2() {
    assert_eq!(part_1("620080001611562C8802118E34"), 12);
}
#[test]
fn test_part_1_example_3() {
    assert_eq!(part_1("C0015000016115A2E0802F182340"), 23);
}
#[test]
fn test_part_1_example_4() {
    assert_eq!(part_1("A0016C880162017C3686B18A3D4780"), 31);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day16.txt");
    assert_eq!(part_1(input), 953);
}

#[test]
fn test_part_2_examples() {
    assert_eq!(part_2("C200B40A82"), 3);
    assert_eq!(part_2("04005AC33890"), 54);
    assert_eq!(part_2("880086C3E88112"), 7);
    assert_eq!(part_2("CE00C43D881120"), 9);
    assert_eq!(part_2("D8005AC2A8F0"), 1);
    assert_eq!(part_2("F600BC2D8F"), 0);
    assert_eq!(part_2("9C005AC2F8F0"), 0);
    assert_eq!(part_2("9C0141080250320F1802104A08"), 1);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day16.txt");
    assert_eq!(part_2(input), 246225449979);
}

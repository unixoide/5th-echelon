use std::fs;
use std::ops::AddAssign;
use std::ops::Shl;
use std::ops::Shr;

use nom::bits::complete::bool;
use nom::bits::complete::tag;
use nom::bits::complete::take;
use nom::multi::count;
use nom::multi::length_count;
use nom::multi::many0;
use nom::IResult;
use nom::Parser as _;

type BitInput<'a> = (&'a [u8], usize);

mod storm {
    macro_rules! enum_from {
        (
            #[repr($ty:ty)]
            pub enum $name:ident {
                $($variant:ident = $value:literal,)*
            }
        ) => {
            #[repr($ty)]
            #[derive(Debug)]
            pub enum $name {
                $($variant = $value,)*
                Unknown($ty),
            }


            impl From<$ty> for $name {
                fn from(value: $ty) -> Self {
                    match value {
                        $(
                            $value => $name::$variant,
                        )*
                        unk => $name::Unknown(unk),
                    }
                }
            }
        };
    }

    enum_from! {
        #[repr(u8)]
        pub enum PacketType {
            MaybeSession = 5,
            Nat = 8,
            PingRequest = 9,
            PingReply = 10,
            MaybePlayer = 11,
        }
    }
}

macro_rules! debug {
    ($e: expr) => {
        // from dbg!():
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $e {
            tmp => {
                println!("[{}:{}] {} = {:#x}", file!(), line!(), stringify!($e), tmp);
                tmp
            }
        }
    };
}

fn hexdump(data: &[u8]) {
    print!("\x1b[34m     ");
    for i in 0..16 {
        print!("{i:2X} ");
    }
    println!("\x1b[m");
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("\x1b[34m{:3x}: ", i * 16);
        for &c in chunk {
            match c {
                0 => print!("\x1b[31m"),
                0xa | 0xd => print!("\x1b[32m"),
                x if x < 0x20 => print!("\x1b[36m"),
                x if x >= 0x7f => print!("\x1b[35m"),
                _ => print!("\x1b[m"),
            }
            print!("{c:02x} ");
        }
        println!("\x1b[m");
    }
}

fn peek_take<O>(count: usize) -> impl Fn(BitInput) -> IResult<BitInput, O>
where
    O: From<u8> + AddAssign + Shl<usize, Output = O> + Shr<usize, Output = O>,
{
    move |input| {
        let (input2, res) = take(count)(input)?;
        let mut end = input.0.len() - input2.0.len() + 1;
        if end > input.0.len() {
            end = input.0.len();
        }
        println!("data = {:#x?}", &input.0[0..end]);
        Ok((input2, res))
    }
}

fn byte(input: BitInput) -> IResult<BitInput, u8> {
    peek_take(8usize)(input).map(|(input, peek)| (input, dbg!(peek)))
}

fn nibble(input: BitInput) -> IResult<BitInput, u8> {
    peek_take(4usize)(input).map(|(input, peek)| (input, dbg!(peek)))
}

fn ushort(input: BitInput) -> IResult<BitInput, u16> {
    peek_take(16usize)(input).map(|(input, peek)| (input, dbg!(peek)))
}

fn uint(input: BitInput) -> IResult<BitInput, u32> {
    peek_take(32usize)(input).map(|(input, peek)| (input, dbg!(peek)))
}

fn ulong(input: BitInput) -> IResult<BitInput, u64> {
    peek_take(64usize)(input).map(|(input, peek)| (input, dbg!(peek)))
}

fn uint_be(input: BitInput) -> IResult<BitInput, u32> {
    let (input, tmp): (BitInput, u32) = peek_take(32usize)(input).map(|(input, peek)| (input, dbg!(peek)))?;
    Ok((input, tmp.swap_bytes()))
}

fn parse_header(input: BitInput) -> IResult<BitInput, u8> {
    let (input, _) = tag(1, 8usize)(input)?;
    let (input, _) = tag(5, 8usize)(input)?;
    let (input, _) = tag(3, 4usize)(input)?;
    let (input, _) = tag(3, 4usize)(input)?;
    let (input, x) = ulong(input)?;
    debug!(x);
    let (input, pty) = nibble(input)?;
    let (input, _) = ushort(input)?;

    Ok((input, pty))
}

fn parse_nat(input: BitInput) -> IResult<BitInput, ()> {
    let (input, x) = nibble(input)?;
    debug!(x);

    Ok((input, ()))
}
fn compressed_ubyte(input: BitInput) -> IResult<BitInput, u8> {
    let (input, tmp) = take_compressed(input, 1, true)?;
    Ok((input, tmp[0]))
}

fn compressed_uint(input: BitInput) -> IResult<BitInput, u32> {
    let (input, tmp) = take_compressed(input, 4, true)?;
    let mut tmp2 = [0u8; 4];
    tmp2.copy_from_slice(&tmp);
    Ok((input, u32::from_le_bytes(tmp2)))
}

fn compressed_uint_be(input: BitInput) -> IResult<BitInput, u32> {
    let (input, tmp) = compressed_uint(input)?;
    Ok((input, tmp.swap_bytes()))
}

fn compressed_ulong(input: BitInput) -> IResult<BitInput, u64> {
    let (input, tmp) = take_compressed(input, 8, true)?;
    let mut tmp2 = [0u8; 8];
    tmp2.copy_from_slice(&tmp);
    Ok((input, u64::from_le_bytes(tmp2)))
}

fn compressed_ulong_be(input: BitInput) -> IResult<BitInput, u64> {
    let (input, tmp) = compressed_ulong(input)?;
    Ok((input, tmp.swap_bytes()))
}

fn take_compressed(mut input: BitInput, num_bytes: usize, is_unsigned: bool) -> IResult<BitInput, Vec<u8>> {
    let mut res = Vec::new();
    let (prefix1, prefix2) = if !is_unsigned {
        let flag;
        (input, flag) = bool(input)?;
        if flag {
            (0xff, 0xf0)
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };
    for i in 1..num_bytes {
        let flag;
        (input, flag) = bool(input)?;
        if !flag {
            let tmp;
            (input, tmp) = count(byte, num_bytes - i + 1).parse(input)?;
            res.extend(tmp);
            return Ok((input, res));
        }
        res.push(prefix1);
    }

    let flag;
    (input, flag) = bool(input)?;
    let mut tmp;
    if flag {
        (input, tmp) = nibble(input)?;
        tmp |= prefix2;
    } else {
        (input, tmp) = byte(input)?;
    }
    res.push(tmp);
    Ok((input, res))
}

fn parse_peer_descriptor(input: BitInput) -> IResult<BitInput, ()> {
    // start field
    let (input, x) = uint(input)?;
    debug!(x);
    let (input, x) = ushort(input)?;
    debug!(x);
    // end field
    // start field
    let (input, x) = compressed_ubyte(input)?;
    println!("x = {x:#x?}");
    // end field
    // start field
    let (input, x) = compressed_uint_be(input)?;
    println!("x = {x:#x?}");
    // end field
    // start field
    let (input, x) = length_count(byte, byte).parse(input)?;
    println!("x = {x:#x?}");
    // end field
    // start field
    let (input, x) = byte(input)?;
    println!("x = {x:#x?}");
    // end field
    // start field
    let (input, x) = uint(input)?;
    debug!(x);
    let (input, x) = ushort(input)?;
    debug!(x);
    // end field
    // start field
    let (input, x) = uint(input)?;
    debug!(x);
    // end field
    Ok((input, ()))
}

fn parse_secure_key(input: BitInput) -> IResult<BitInput, ()> {
    let (input, x) = ulong(input)?;
    debug!(x);
    Ok((input, ()))
}

fn parse_routable(input: BitInput) -> IResult<BitInput, ()> {
    let (input, x) = uint(input)?;
    dbg!(x);

    // start peer descriptor
    let (input, _) = parse_peer_descriptor(input)?;

    // some conditional parsing
    let (input, flag) = bool(input)?;
    let (input, _) = if flag {
        // 0200ca0e
        parse_peer_descriptor(input)?
    } else {
        (input, ())
    };

    // start secure key
    let (input, _) = parse_secure_key(input)?;
    // end secure key

    Ok((input, ()))
}

fn packet_type2peer_packet_type(packet_type: u8) -> u8 {
    match packet_type {
        1 | 2 | 6 | 0xc => 1,
        3 | 4 | 7 => 0,
        5 | 0xb => 3,
        _ => unreachable!(),
    }
}

fn parse_peer_packet(input: BitInput, packet_type: u8) -> IResult<BitInput, ()> {
    let (input, _) = parse_routable(input)?;

    let (mut input, x) = uint(input)?;
    debug!(x);

    let peer_packet_type = packet_type2peer_packet_type(packet_type);

    if peer_packet_type != 0 {
        let mut x;
        (input, x) = uint(input)?;
        debug!(x);
        (input, x) = uint(input)?;
        debug!(x);

        if peer_packet_type == 3 {
            let x;
            (input, x) = ushort(input)?;
            debug!(x);
        }
    }
    let x;
    (input, x) = uint(input)?;
    debug!(x);
    let x;
    (input, x) = ushort(input)?;
    debug!(x);
    let x;
    (input, x) = ushort(input)?;
    debug!(x);
    let x;
    (input, x) = byte(input)?;
    debug!(x);
    let size;
    (input, size) = ushort(input)?;
    debug!(size);

    // later parsing
    if packet_type == 1 {
        let x;
        (input, x) = uint(input)?;
        debug!(x);
        // FUN_020b7100
        let x;
        (input, x) = uint(input)?;
        debug!(x);
        let x;
        (input, x) = uint(input)?;
        debug!(x);
        // FUN_00a873d0
        let x;
        (input, x) = compressed_ulong_be(input)?;
        debug!(x);
        // FUN_00a86c80
        let x;
        (input, x) = uint(input)?;
        debug!(x);
    }
    Ok((input, ()))
}

fn parse_packet(input: BitInput) -> IResult<BitInput, ()> {
    let (input, packet_type) = parse_header(input)?;
    dbg!(storm::PacketType::from(dbg!(packet_type)));

    let (input, _payload) = match packet_type {
        1 => parse_peer_packet(input, packet_type)?,
        8 => parse_nat(input)?,
        _ => unimplemented!(),
    };

    Ok((input, ()))
}

fn take_rest(input: BitInput) -> IResult<BitInput, Vec<u8>> {
    let (mut input, mut res) = many0(byte).parse(input)?;
    if input.1 > 0 {
        let tmp;
        (input, tmp) = take(8 - input.1)(input)?;
        res.push(tmp);
    }
    Ok((input, res))
}

fn main() {
    let data = fs::read(std::env::args().nth(1).expect("filename")).expect("readable file");
    let input = (data.as_slice(), 0);
    let (input, _out) = parse_packet(input).unwrap();
    println!("parsed bits = {:#x}", (data.len() - input.0.len()) * 8 + input.1);
    hexdump(&take_rest(input).expect("take_rest").1);
}

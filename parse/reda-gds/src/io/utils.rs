

pub fn bytes_to_f64(buf: [u8; 8]) -> f64 {
    let data = u64::from_be_bytes(buf); 

    let sign = (data >> 63) & 0x1;
    let mut exponent = (data >> 56) & 0x7F;
    let mut mantissa = data << 8; 

    if mantissa == 0 {
        return 0.0;
    }

    exponent = ((exponent as i64 - 64) * 4 + 1023) as u64;

    while (mantissa & 0x8000_0000_0000_0000) == 0 {
        mantissa <<= 1;
        exponent -= 1;
    }

    mantissa <<= 1;
    exponent -= 1;

    let ieee_bits =
        (sign << 63) |
        (exponent << 52) |
        ((mantissa >> 12) & 0x000F_FFFF_FFFF_FFFF);

    f64::from_bits(ieee_bits)
}

pub fn f64_to_bytes(value: f64) -> [u8; 8] {
    let data = value.to_bits() as i64;

    let mut sign = (data >> 63) & 0x1;
    let mut exponent = ((data >> 52) & 0x7FF) - 1023;
    let mut mantissa = data << 12;

    if data == 0 {
        sign = 0;
        exponent = 0;
        mantissa = 0;
    } else {
        mantissa >>= 1;
        mantissa = mantissa | 0x8000000000000000u64 as i64;
        exponent += 1;

        let count = (-1 * exponent as i64) & 0x3;
        for _ in 0..count {
            mantissa >>= 1;
            mantissa = mantissa & 0x7fffffffffffffff;
        }

        exponent = (exponent + 3) >> 2;
        exponent += 64;
    }

    let ibm = ((sign << 63) | (exponent << 56) | ((mantissa >> 8) & 0x00ff_ffff_ffff_ffff)) as u64;
    let ibm_bytes = ibm.to_be_bytes();

    ibm_bytes
}

#[cfg(test)]
mod test {
    use super::{bytes_to_f64, f64_to_bytes};

    #[test]
    fn test_trans() {
        let bytes = f64_to_bytes(1e-9);
        let value = bytes_to_f64(bytes);
        println!("{}", value);
    }
}
use std::io::Read;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::ToPrimitive;
use num_traits::Zero;

use crate::model::AString;
use crate::model::BString;
use crate::model::Delta1;
use crate::model::Delta2;
use crate::model::Delta3;
use crate::model::Direction1;
use crate::model::Direction2;
use crate::model::Direction3;
use crate::model::GDelta;
use crate::model::NString;
use crate::model::PointList;
use crate::model::Real;
use crate::model::Repetition;

use super::{ReadError, ReadResult};

pub fn read_unsigned_integer<R: Read>(stream: &mut R) -> ReadResult<BigUint> {
    let mut result = BigUint::zero();
    let mut shift = 0u32;
    let mut byte = [0u8; 1];

    loop {
        stream.read_exact(&mut byte)?;
        let b = byte[0];
        let chunk = BigUint::from(remove_marker(b));
        result |= chunk << shift;
        if is_last_byte(b) {
            break;
        }
        shift += 7;
    }

    Ok(result)
}

pub fn read_integer<R: Read>(stream: &mut R) -> ReadResult<BigInt> {
    #[inline]
    fn get_sign(byte: u8) -> Sign {
        if byte & 0b0000_0001 == 0 { Sign::Plus } else { Sign::Minus }
    }
    
    #[inline]
    fn remove_sign_marker(byte: u8) -> u8 {
        (byte >> 1) & 0b0011_1111
    }

    let mut result = BigUint::zero();
    let mut shift = 0u32;
    let mut byte = [0u8; 1];
    let mut first_byte = true;
    let mut sign = Sign::Plus;

    loop {
        stream.read_exact(&mut byte)?;
        let b = byte[0];
        if first_byte {
            first_byte = false;
            sign = get_sign(b);
            let chunk = BigUint::from(remove_sign_marker(b));
            result |= chunk << shift;
            shift += 6;
        } else {
            let chunk = BigUint::from(remove_marker(b));
            result |= chunk << shift;
            shift += 7;
        }
        if is_last_byte(b) {
            break;
        }
    }

    Ok(BigInt::from_biguint(sign, result))
}

pub fn read_u8<R: Read>(stream: &mut R) -> ReadResult<u8> {
    let ui = read_unsigned_integer(stream)?;
    ui.to_u8().ok_or_else(|| ReadError::UnsignedIntOutOfRangeU8)
}

pub fn read_real<R: Read>(stream: &mut R) -> ReadResult<Real> {
    let value = read_unsigned_integer(stream)?;
    let kind = value.clone().to_u8()
        .ok_or_else(|| ReadError::UnsupportRealType(value.clone()))?;

    match kind {
        0 => Ok(Real::PositiveWhole(read_unsigned_integer(stream)?)),
        1 => Ok(Real::NegativeWhole(read_unsigned_integer(stream)?)),
        2 => Ok(Real::PositiveReciprocal(read_unsigned_integer(stream)?)),
        3 => Ok(Real::NegativeReciprocal(read_unsigned_integer(stream)?)),
        4 => Ok(Real::PositiveRatio(read_unsigned_integer(stream)?, read_unsigned_integer(stream)?)),
        5 => Ok(Real::NegativeRatio(read_unsigned_integer(stream)?, read_unsigned_integer(stream)?)),
        6 => Ok(Real::SinglePrecisionFloat(read_f32(stream)?)),
        7 => Ok(Real::DoublePrecisionFloat(read_f64(stream)?)),
        _ => Err(ReadError::UnsupportRealType(value)),
    }
}

pub fn read_bytes<const C: usize, R: Read>(stream: &mut R) -> ReadResult<[u8; C]> {
    let mut bytes = [0u8; C];
    stream.read_exact(&mut bytes)?;
    Ok(bytes)
}

fn read_string_length<R: Read>(stream: &mut R) -> ReadResult<usize> {
    let length_big: BigUint = read_unsigned_integer(stream)?;
    let length = length_big.to_usize()
        .ok_or_else(|| ReadError::StringTooLong(length_big))?;
    Ok(length)
}

pub fn read_b_string<R: Read>(stream: &mut R) -> ReadResult<BString> {
    let length = read_string_length(stream)?;
    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer)?;
    Ok(BString(buffer))
}

pub fn read_a_string<R: Read>(stream: &mut R) -> ReadResult<AString> {
    let length = read_string_length(stream)?;
    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer)?;

    // 校验 ASCII 范围
    for &byte in &buffer {
        if !(byte == 0x20 || (0x21..=0x7E).contains(&byte)) {
            return Err(ReadError::InvalidAStringByte(byte));
        }
    }

    let s = String::from_utf8(buffer)?;
    Ok(AString(s))
}

pub fn read_n_string<R: Read>(stream: &mut R) -> ReadResult<NString> {
    let length = read_string_length(stream)?;
    if length == 0 {
        return Err(ReadError::EmptyNString);
    }

    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer)?;

    // 校验 ASCII 范围
    for &byte in &buffer {
        if !(0x21..=0x7E).contains(&byte) {
            return Err(ReadError::InvalidNStringByte(byte));
        }
    }

    let s = String::from_utf8(buffer)?;
    Ok(NString(s))
}

fn bit_to_direction1(bit: bool) -> Direction1 {
    if bit { Direction1::Negative } else { Direction1::Positive }
}

pub fn read_delta1<R: Read>(stream: &mut R) -> ReadResult<Delta1> {
    let n = read_unsigned_integer(stream)?;
    let direction = bit_to_direction1(n.bit(0));
    let magnitude = n >> 1;
    Ok(Delta1 { magnitude, direction })

}

pub fn read_delta2<R: Read>(stream: &mut R) -> ReadResult<Delta2> {
    let n = read_unsigned_integer(stream)?;
    let direction = match (n.bit(1), n.bit(0)) {
        (false, false) => Direction2::East,
        (false, true)  => Direction2::North,
        (true, false)  => Direction2::West,
        (true, true)   => Direction2::South,
    };
    
    let magnitude = n >> 2;
    Ok(Delta2 { magnitude, direction })
}

pub fn read_delta3<R: Read>(stream: &mut R) -> ReadResult<Delta3> {
    let n = read_unsigned_integer(stream)?;
    let direction = match (n.bit(2), n.bit(1), n.bit(0)) {
        (false, false, false) => Direction3::East,
        (false, false, true)  => Direction3::North,
        (false, true, false)  => Direction3::West,
        (false, true, true)   => Direction3::South,
        (true, false, false)  => Direction3::Northeast,
        (true, false, true)   => Direction3::Northwest,
        (true, true, false)   => Direction3::Southwest,
        (true, true, true)    => Direction3::Southeast,
    };
    
    let magnitude = n >> 3;
    Ok(Delta3 { magnitude, direction })
}

pub fn read_gdelta<R: Read>(stream: &mut R) -> ReadResult<GDelta> {
    let n1 = read_unsigned_integer(stream)?;
    if !n1.bit(0) {
        // g1
        let direction = match (n1.bit(3), n1.bit(2), n1.bit(1)) {
            (false, false, false) => Direction3::East,
            (false, false, true)  => Direction3::North,
            (false, true, false)  => Direction3::West,
            (false, true, true)   => Direction3::South,
            (true, false, false)  => Direction3::Northeast,
            (true, false, true)   => Direction3::Northwest,
            (true, true, false)   => Direction3::Southwest,
            (true, true, true)    => Direction3::Southeast,
        };
        let magnitude = n1 >> 4;

        Ok(GDelta::G1 { magnitude, direction })
    } else {
        // g2
        let x_dir = bit_to_direction1(n1.bit(1));
        let x = n1 >> 2;
        let n2 = read_unsigned_integer(stream)?;
        let y_dir = bit_to_direction1(n2.bit(0));
        let y = n2 >> 1;

        Ok(GDelta::G2 { x, x_dir, y, y_dir})
    }
}

pub fn read_repetition<R: Read>(stream: &mut R) -> ReadResult<Repetition> {
    fn check_dimension(dimension: &BigUint) -> ReadResult<()> {
        if dimension.is_zero() {
            return Err(ReadError::RepetitionDimensionZero);
        }
        Ok(())
    }

    let value = read_unsigned_integer(stream)?;
    let kind = value.clone().to_u8()
        .ok_or_else(|| ReadError::UnsupportRepetitionType(value.clone()))?;

    match kind {
        0 => Ok(Repetition::Reuse),
        1 => {
            let x_dimension = read_unsigned_integer(stream)?;
            check_dimension(&x_dimension)?;
            let y_dimension = read_unsigned_integer(stream)?;
            check_dimension(&y_dimension)?;
            let x_space = read_unsigned_integer(stream)?;
            let y_space = read_unsigned_integer(stream)?;
            Ok(Repetition::UniformMatrix { x_dimension, y_dimension, x_space, y_space })
        }
        2 => {
            let x_dimension = read_unsigned_integer(stream)?;
            check_dimension(&x_dimension)?;
            let x_space = read_unsigned_integer(stream)?;
            Ok(Repetition::UniformRow { x_dimension, x_space })
        }
        3 => {
            let y_dimension = read_unsigned_integer(stream)?;
            check_dimension(&y_dimension)?;
            let y_space = read_unsigned_integer(stream)?;
            Ok(Repetition::UniformColumn { y_dimension, y_space })
        }
        4 => {
            let x_dimension = read_unsigned_integer(stream)?;
            check_dimension(&x_dimension)?;
            let space_len = x_dimension.to_usize().ok_or_else(|| ReadError::DimensionTooLong(x_dimension.clone()))?;
            let mut x_spaces = vec![];
            for _ in 1..space_len {
                x_spaces.push(read_unsigned_integer(stream)?);
            }
            Ok(Repetition::VariableRow { x_dimension, x_spaces })
        }
        5 => {
            let x_dimension = read_unsigned_integer(stream)?;
            check_dimension(&x_dimension)?;
            let grid = read_unsigned_integer(stream)?;
            let space_len = x_dimension.to_usize().ok_or_else(|| ReadError::DimensionTooLong(x_dimension.clone()))?;
            let mut x_spaces = vec![];
            for _ in 1..space_len {
                x_spaces.push(read_unsigned_integer(stream)?);
            }
            Ok(Repetition::VariableRowGrid { x_dimension, x_spaces, grid})
        }
        6 => {
            let y_dimension = read_unsigned_integer(stream)?;
            check_dimension(&y_dimension)?;
            let space_len = y_dimension.to_usize().ok_or_else(|| ReadError::DimensionTooLong(y_dimension.clone()))?;
            let mut y_spaces = vec![];
            for _ in 1..space_len {
                y_spaces.push(read_unsigned_integer(stream)?);
            }
            Ok(Repetition::VariableColumn { y_dimension, y_spaces })
        }
        7 => {
            let y_dimension = read_unsigned_integer(stream)?;
            check_dimension(&y_dimension)?;
            let grid = read_unsigned_integer(stream)?;
            let space_len = y_dimension.to_usize().ok_or_else(|| ReadError::DimensionTooLong(y_dimension.clone()))?;
            let mut y_spaces = vec![];
            for _ in 1..space_len {
                y_spaces.push(read_unsigned_integer(stream)?);
            }
            Ok(Repetition::VariableColumnGrid { y_dimension, grid, y_spaces })
        }
        8 => {
            let n_dimension = read_unsigned_integer(stream)?;
            check_dimension(&n_dimension)?;
            let m_dimension = read_unsigned_integer(stream)?;
            check_dimension(&m_dimension)?;
            let n_displacement = read_gdelta(stream)?;
            let m_displacement = read_gdelta(stream)?;
            Ok(Repetition::MatrixDisplacement { n_dimension, m_dimension, n_displacement, m_displacement })
        }
        9 => {
            let dimension = read_unsigned_integer(stream)?;
            check_dimension(&dimension)?;
            let displacement = read_gdelta(stream)?;
            Ok(Repetition::VectorDisplacement { dimension, displacement })
        }
        10 => {
            let dimension = read_unsigned_integer(stream)?;
            check_dimension(&dimension)?;
            let len = dimension.to_usize().ok_or_else(|| ReadError::DimensionTooLong(dimension.clone()))?;
            let mut displacements = vec![];
            for _ in 1..len {
                displacements.push(read_gdelta(stream)?);
            }

            Ok(Repetition::VariableVector { dimension, displacements })
        }
        11 => {
            let dimension = read_unsigned_integer(stream)?;
            check_dimension(&dimension)?;
            let grid = read_unsigned_integer(stream)?;

            let len = dimension.to_usize().ok_or_else(|| ReadError::DimensionTooLong(dimension.clone()))?;
            let mut displacements = vec![];
            for _ in 1..len {
                displacements.push(read_gdelta(stream)?);
            }

            Ok(Repetition::VariableVectorGrid { dimension, grid, displacements })
        }
        _ => Err(ReadError::UnsupportRepetitionType(value)),
    }
}

pub fn read_point_list<R: Read>(stream: &mut R) -> ReadResult<PointList> {
    let value = read_unsigned_integer(stream)?;
    let kind = value.clone().to_u8()
        .ok_or_else(|| ReadError::UnsupportPointListType(value.clone()))?;
    let vertex_count_bigunit = read_unsigned_integer(stream)?;
    let vertex_count = vertex_count_bigunit.to_usize()
        .ok_or_else(|| ReadError::InvalidVertexCount(vertex_count_bigunit.clone(), "too mauch vertex count"))?;

    match kind {
        0 => {
            // AlternatingHV: final 2 deltas omitted for polygons
            if vertex_count < 2 {
                return Err(ReadError::InvalidVertexCount(vertex_count_bigunit, "AlternatingHV's vertex size less 2"));
            }
            let mut vertexs = vec![];
            for _ in 2..vertex_count {
                vertexs.push(read_delta1(stream)?);
            }
            Ok(PointList::AlternatingHV(vertexs))
        }
        1 => {
            // AlternatingVH: final 2 deltas omitted for polygons
            if vertex_count < 2 {
                return Err(ReadError::InvalidVertexCount(vertex_count_bigunit, "AlternatingVH's vertex size less than 2"));
            }
            let delta_count = vertex_count - 2;
            let mut deltas = Vec::with_capacity(delta_count);
            for _ in 0..delta_count {
                deltas.push(read_delta1(stream)?);
            }
            Ok(PointList::AlternatingVH(deltas))
        }
        2 => {
            // Manhattan: final delta omitted
            if vertex_count < 1 {
                return Err(ReadError::InvalidVertexCount(vertex_count_bigunit, "Manhattan vertex count too small"));
            }
            let delta_count = vertex_count - 1;
            let mut deltas = Vec::with_capacity(delta_count);
            for _ in 0..delta_count {
                deltas.push(read_delta2(stream)?);
            }
            Ok(PointList::Manhattan(deltas))
        }
        3 => {
            // Octangular: final delta omitted
            if vertex_count < 1 {
                return Err(ReadError::InvalidVertexCount(vertex_count_bigunit, "Octangular vertex count too small"));
            }
            let delta_count = vertex_count - 1;
            let mut deltas = Vec::with_capacity(delta_count);
            for _ in 0..delta_count {
                deltas.push(read_delta3(stream)?);
            }
            Ok(PointList::Octangular(deltas))
        }
        4 => {
            // AnyAngle: final delta omitted
            if vertex_count < 1 {
                return Err(ReadError::InvalidVertexCount(vertex_count_bigunit, "AnyAngle vertex count too small"));
            }
            let delta_count = vertex_count - 1;
            let mut deltas = Vec::with_capacity(delta_count);
            for _ in 0..delta_count {
                deltas.push(read_gdelta(stream)?);
            }
            Ok(PointList::AnyAngle(deltas))
        }
        5 => {
            // CumulativeAnyAngle: final delta omitted
            if vertex_count < 1 {
                return Err(ReadError::InvalidVertexCount(vertex_count_bigunit, "CumulativeAnyAngle vertex count too small"));
            }
            let delta_count = vertex_count - 1;
            let mut deltas = Vec::with_capacity(delta_count);
            for _ in 0..delta_count {
                deltas.push(read_gdelta(stream)?);
            }
            Ok(PointList::CumulativeAnyAngle(deltas))
        }
        _ => Err(ReadError::UnsupportPointListType(value.clone())),
    }
}

fn read_f32<R: Read>(reader: &mut R) -> ReadResult<f32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

fn read_f64<R: Read>(reader: &mut R) -> ReadResult<f64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(f64::from_le_bytes(buf))
}

#[inline]
fn is_last_byte(byte: u8) -> bool {
    byte & 0b1000_0000 == 0
}

#[inline]
fn remove_marker(byte: u8) -> u8 {
    byte & 0b0111_1111
}

#[cfg(test)]
mod tests {
    use crate::model::Delta2D;

    use super::*;
    use std::io::Cursor;
    use num_bigint::{BigUint, BigInt, Sign};
    use num_traits::{FromPrimitive, ToPrimitive, One};

    fn decode_unsigned(data: &[u8]) -> u64 {
        let mut cursor = Cursor::new(data);
        read_unsigned_integer(&mut cursor)
            .unwrap()
            .to_u64()
            .unwrap()
    }

    fn decode_signed(data: &[u8]) -> i64 {
        let mut cursor = Cursor::new(data);
        read_integer(&mut cursor)
            .unwrap()
            .to_i64()
            .unwrap()
    }

    #[test]
    fn test_single_byte_unsigned() {
        for n in 0u8..=127 {
            assert_eq!(decode_unsigned(&[n]), n as u64);
        }
    }

    #[test]
    fn test_unsigned_boundaries() {
        assert_eq!(decode_unsigned(&[0x7F]), 127);
        assert_eq!(decode_unsigned(&[0x80, 0x01]), 128);
        assert_eq!(decode_unsigned(&[0xFF, 0x01]), 255);
        assert_eq!(decode_unsigned(&[0x80, 0x80, 0x01]), 16384);
        assert_eq!(decode_unsigned(&[0xFF, 0xFF, 0x7F]), 2_097_151);
        assert_eq!(decode_unsigned(&[0xFF, 0x7F]), 16_383);
    }

    #[test]
    fn test_unsigned_multi_byte_grow() {
        assert_eq!(decode_unsigned(&[0x80, 0x80, 0x80, 0x01]), 2u64.pow(21));
        assert_eq!(decode_unsigned(&[0xFF, 0xFF, 0xFF, 0x7F]), (1 << 28) - 1);
        assert_eq!(decode_unsigned(&[0x80, 0x80, 0x80, 0x80, 0x01]), 1 << 28);
    }

    #[test]
    fn test_large_unsigned_biguint() {
        let value = BigUint::from_u128(1u128 << 100).unwrap();
        let encoded = encode_unsigned_integer(&value);
        let mut cursor = Cursor::new(encoded);
        let decoded = read_unsigned_integer(&mut cursor).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_unsigned_roundtrip_various() {
        let test_values = [
            0u64,
            1,
            127,
            128,
            255,
            16383,
            16384,
            2_097_151,
            1234567,
            9_999_999,
            u32::MAX as u64,
        ];
        for &n in &test_values {
            let value = BigUint::from_u64(n).unwrap();
            let enc = encode_unsigned_integer(&value);
            let mut c = Cursor::new(enc.clone());
            let decoded = read_unsigned_integer(&mut c).unwrap();
            assert_eq!(decoded, value, "failed at {:?}", enc);
        }
    }

    fn encode_unsigned_integer(value: &BigUint) -> Vec<u8> {
        let mut result = Vec::new();
        let mut v = value.clone();
        let mask = BigUint::from_u8(0x7F).unwrap();
        while &v > &mask {
            let byte = ((&v & &mask).to_u8().unwrap()) | 0x80;
            result.push(byte);
            v >>= 7;
        }
        result.push(v.to_u8().unwrap());
        result
    }

    #[test]
    fn test_basic_signed() {
        assert_eq!(decode_signed(&[0x00]), 0);
        assert_eq!(decode_signed(&[0x01]), 0);
        assert_eq!(decode_signed(&[0x02]), 1);
        assert_eq!(decode_signed(&[0x03]), -1);
        assert_eq!(decode_signed(&[0x04]), 2);
        assert_eq!(decode_signed(&[0x05]), -2);
        assert_eq!(decode_signed(&[0x7E]), 63);
        assert_eq!(decode_signed(&[0x7F]), -63);
    }

    #[test]
    fn test_signed_multi_byte() {
        assert_eq!(decode_signed(&[0x80, 0x01]), 64);
        assert_eq!(decode_signed(&[0x81, 0x01]), -64);
        assert_eq!(decode_signed(&[0xFE, 0x7F]), 8191);
        assert_eq!(decode_signed(&[0xFF, 0x7F]), -8191);
        assert_eq!(decode_signed(&[0x80, 0x80, 0x01]), 8192);
        assert_eq!(decode_signed(&[0x81, 0x80, 0x01]), -8192);
    }

    #[test]
    fn test_signed_large_values() {
        /*
            1100_0000, 1000_0100, 0011_1101  
            0b100_000 + 0b000_0100 * 2**6 + 0b011_1101 * 2**13
        */
        assert_eq!(decode_signed(&[0xC0, 0x84, 0x3D]), 500000);
        assert_eq!(decode_signed(&[0xC1, 0x84, 0x3D]), -500000);
    }

    #[test]
    fn test_signed_roundtrip_various() {
        use rand::Rng;

        fn encode_signed_integer(v: &BigInt) -> Vec<u8> {
            let (sign, abs) = v.clone().into_parts();
            let mut val = &abs << 1;
            if sign == Sign::Minus {
                val += BigUint::one();
            }
            encode_unsigned_integer(&val)
        }

        let cases = [
            -10,
            -1,
            0,
            1,
            2,
            63,
            64,
            8191,
            -8191,
            1_000_000,
            -1_000_000,
        ];
        for &n in &cases {
            let val = BigInt::from_i64(n).unwrap();
            let enc = encode_signed_integer(&val);
            let mut c = Cursor::new(enc.clone());
            let dec = read_integer(&mut c).unwrap();
            assert_eq!(dec, val, "round-trip failed for {:?} → {:?}", n, enc);
        }

        // 随机测试
        let mut rng = rand::rng();
        for _ in 0..200 {
            let n: i64 = rng.random_range(-5_000_000..5_000_000);
            let val = BigInt::from_i64(n).unwrap();
            let enc = encode_signed_integer(&val);
            let mut c = Cursor::new(enc.clone());
            let dec = read_integer(&mut c).unwrap();
            assert_eq!(dec, val, "random round-trip failed for {}", n);
        }
    }

    // 位模式显式验证
    #[test]
    fn test_signed_bit_patterns() {
        // 测试符号位正确移除：最低位为符号
        // abs=1 (0b1<<1 | sign=0) => 0b10
        assert_eq!(decode_signed(&[0x02]), 1);
        // abs=1, sign=1 => 0b11
        assert_eq!(decode_signed(&[0x03]), -1);
        // abs=64, sign=0 => 0b1000_0000_0000_0000
        assert_eq!(decode_signed(&[0x80, 0x01]), 64);
        // abs=64, sign=1 => 0b1000_0000_0000_0001
        assert_eq!(decode_signed(&[0x81, 0x01]), -64);
    }

    fn make_real_bytes(kind: u8, values: &[BigUint]) -> Vec<u8> {
        let mut bytes = encode_unsigned_integer(&BigUint::from_u8(kind).unwrap());
        for v in values {
            bytes.extend(encode_unsigned_integer(v));
        }
        bytes
    }

    #[test]
    fn test_positive_whole() {
        let value = BigUint::from_u32(12345).unwrap();
        let data = make_real_bytes(0, &[value.clone()]);
        let mut cursor = Cursor::new(data);
        let r = read_real(&mut cursor).unwrap();
        assert_eq!(r, Real::PositiveWhole(value));
    }

    #[test]
    fn test_negative_whole() {
        let value = BigUint::from_u32(54321).unwrap();
        let data = make_real_bytes(1, &[value.clone()]);
        let mut cursor = Cursor::new(data);
        let r = read_real(&mut cursor).unwrap();
        assert_eq!(r, Real::NegativeWhole(value));
    }

    #[test]
    fn test_positive_reciprocal() {
        let value = BigUint::from_u32(7).unwrap();
        let data = make_real_bytes(2, &[value.clone()]);
        let mut cursor = Cursor::new(data);
        let r = read_real(&mut cursor).unwrap();
        assert_eq!(r, Real::PositiveReciprocal(value));
    }

    #[test]
    fn test_negative_reciprocal() {
        let value = BigUint::from_u32(13).unwrap();
        let data = make_real_bytes(3, &[value.clone()]);
        let mut cursor = Cursor::new(data);
        let r = read_real(&mut cursor).unwrap();
        assert_eq!(r, Real::NegativeReciprocal(value));
    }

    #[test]
    fn test_positive_ratio() {
        let numerator = BigUint::from_u32(2).unwrap();
        let denominator = BigUint::from_u32(3).unwrap();
        let data = make_real_bytes(4, &[numerator.clone(), denominator.clone()]);
        let mut cursor = Cursor::new(data);
        let r = read_real(&mut cursor).unwrap();
        assert_eq!(r, Real::PositiveRatio(numerator, denominator));
    }

    #[test]
    fn test_negative_ratio() {
        let numerator = BigUint::from_u32(5).unwrap();
        let denominator = BigUint::from_u32(8).unwrap();
        let data = make_real_bytes(5, &[numerator.clone(), denominator.clone()]);
        let mut cursor = Cursor::new(data);
        let r = read_real(&mut cursor).unwrap();
        assert_eq!(r, Real::NegativeRatio(numerator, denominator));
    }

    #[test]
    fn test_single_precision_float() {
        use std::f32::consts::PI;
        let mut bytes = encode_unsigned_integer(&BigUint::from_u8(6).unwrap());
        bytes.extend(&PI.to_le_bytes());
        let mut cursor = Cursor::new(bytes);
        let r = read_real(&mut cursor).unwrap();
        match r {
            Real::SinglePrecisionFloat(f) => assert!((f - PI).abs() < 1e-6),
            _ => panic!("Expected SinglePrecisionFloat"),
        }
    }

    #[test]
    fn test_double_precision_float() {
        use std::f64::consts::E;
        let mut bytes = encode_unsigned_integer(&BigUint::from_u8(7).unwrap());
        bytes.extend(&E.to_le_bytes());
        let mut cursor = Cursor::new(bytes);
        let r = read_real(&mut cursor).unwrap();
        match r {
            Real::DoublePrecisionFloat(f) => assert!((f - E).abs() < 1e-12),
            _ => panic!("Expected DoublePrecisionFloat"),
        }
    }

    #[test]
    fn test_unsupported_kind() {
        let bytes = encode_unsigned_integer(&BigUint::from_u8(255).unwrap());
        let mut cursor = Cursor::new(bytes);
        let err = read_real(&mut cursor).unwrap_err();
        match err {
            ReadError::UnsupportRealType(_) => {}
            _ => panic!("Expected UnsupportRealType error"),
        }
    }

    #[test]
    fn test_delta1() {
        let bytes: Vec<u8> = vec![0b11111001, 0b00100011];
        let mut cursor = Cursor::new(bytes);
        let delta = read_delta1(&mut cursor).unwrap();
        assert_eq!(delta.magnitude, BigUint::from(2300u32));
        assert_eq!(delta.direction, Direction1::Negative);

        let bytes: Vec<u8> = vec![0b11111000, 0b00100011];
        let mut cursor = Cursor::new(bytes);
        let delta = read_delta1(&mut cursor).unwrap();
        assert_eq!(delta.magnitude, BigUint::from(2300u32));
        assert_eq!(delta.direction, Direction1::Positive);
    }

    #[test]
    fn test_delta2() {
        let bytes: Vec<u8> = vec![0b10011000, 0b00101010];
        let mut cursor = Cursor::new(bytes);
        let delta = read_delta2(&mut cursor).unwrap();
        assert_eq!(delta.magnitude, BigUint::from(1350u32));
        assert_eq!(delta.direction, Direction2::East);
        assert_eq!(delta.delta_x(), BigInt::from(1350));
        assert_eq!(delta.delta_y(), BigInt::zero());

        let bytes: Vec<u8> = vec![0b10011011, 0b00101010];
        let mut cursor = Cursor::new(bytes);
        let delta = read_delta2(&mut cursor).unwrap();
        assert_eq!(delta.magnitude, BigUint::from(1350u32));
        assert_eq!(delta.direction, Direction2::South);
        assert_eq!(delta.delta_y(), BigInt::from(-1350));
        assert_eq!(delta.delta_x(), BigInt::zero());
    }

    #[test]
    fn test_delta3() {
        let bytes: Vec<u8> = vec![0b11001101, 0b00000001];
        let mut cursor = Cursor::new(bytes);
        let delta = read_delta3(&mut cursor).unwrap();
        assert_eq!(delta.magnitude, BigUint::from(25u32));
        assert_eq!(delta.direction, Direction3::Northwest);
        assert_eq!(delta.delta_x(), BigInt::from(-25));
        assert_eq!(delta.delta_y(), BigInt::from(25));        

        let bytes: Vec<u8> = vec![0b11010111, 0b00000111];
        let mut cursor = Cursor::new(bytes);
        let delta = read_delta3(&mut cursor).unwrap();
        assert_eq!(delta.magnitude, BigUint::from(122u32));
        assert_eq!(delta.direction, Direction3::Southeast);
        assert_eq!(delta.delta_x(), BigInt::from(122));
        assert_eq!(delta.delta_y(), BigInt::from(-122));       
    }

    #[test]
    fn test_gdelta() {
        let bytes: Vec<u8> = vec![0b11101001, 0b00000011, 0b01111010];
        let mut cursor = Cursor::new(bytes);
        let delta = read_gdelta(&mut cursor).unwrap();
        assert_eq!(delta.delta_x(), BigInt::from(122));
        assert_eq!(delta.delta_y(), BigInt::from(61));  
        if let GDelta::G2 { x, x_dir, y, y_dir }= delta {
            assert_eq!(x, BigUint::from(122u32));
            assert_eq!(x_dir, Direction1::Positive);
            assert_eq!(y, BigUint::from(61u32));
            assert_eq!(y_dir, Direction1::Positive); 
        } else {
            assert!(false);
        }

        let bytes: Vec<u8> = vec![0b10111011, 0b00000001, 0b10110111, 0b00001111];
        let mut cursor = Cursor::new(bytes);
        let delta = read_gdelta(&mut cursor).unwrap();
        assert_eq!(delta.delta_x(), BigInt::from(-46));
        assert_eq!(delta.delta_y(), BigInt::from(-987));  
        if let GDelta::G2 { x, x_dir, y, y_dir }= delta {
            assert_eq!(x, BigUint::from(46u32));
            assert_eq!(x_dir, Direction1::Negative);
            assert_eq!(y, BigUint::from(987u32));
            assert_eq!(y_dir, Direction1::Negative);
        } else {
            assert!(false);
        }

        let bytes: Vec<u8> = vec![0b11101100, 0b00000101];
        let mut cursor = Cursor::new(bytes);
        let delta = read_gdelta(&mut cursor).unwrap();
        assert_eq!(delta.delta_x(), BigInt::from(-46));
        assert_eq!(delta.delta_y(), BigInt::from(-46));  
        if let GDelta::G1 { magnitude, direction } = delta {
            assert_eq!(magnitude, BigUint::from(46u32));
            assert_eq!(direction, Direction3::Southwest);
        }
    }
}

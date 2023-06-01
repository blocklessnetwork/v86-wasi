
lazy_static::lazy_static! {
    static ref INT_LOG2_TABLE: [i32; 256] = {
        let mut table = [0; 256];
        let mut b = -2;
        for i in 0..256 {
            if (i & i - 1) == 0 {
                b += 1;
            }
            table[i] = b;
        }
        table
    };
}

#[inline]
pub fn int_log2_byte(i: u8) -> i32 {
    INT_LOG2_TABLE[i as usize]
}

pub fn int_log2(x: i32) -> i32 {
    let mut x: u32 = x as u32;
    assert!(x > 0);

    // http://jsperf.com/integer-log2/6
    let tt = x >> 16;
    if tt > 0 {
        let t = tt >> 8;
        if t > 0 {
            return 24 + INT_LOG2_TABLE[t as usize];
        } else {
            return 16 + INT_LOG2_TABLE[tt as usize];
        }
    } else {
        let t = x >> 8;
        if t > 0 {
            return 8 + INT_LOG2_TABLE[t as usize];
        } else {
            return INT_LOG2_TABLE[x as usize];
        }
    }
}


macro_rules! copy_impl {
    ($name: ident, $type: ty, $l: literal) => {
        pub fn $name(src: &[u8], dst: &mut [$type]) {
            let mut bs = [0u8; $l];
            for i in 0..dst.len() {
                let start = i * $l;
                let end = start + $l;
                bs.copy_from_slice(&src[start..end]);
                let t: $type = <$type>::from_le_bytes(bs);
                dst[i] = t;
            }
        }
    };
}

macro_rules! read_impl {
    ($name: ident, $type: ty, $l: literal) => {
        pub fn $name(src: &[u8], idx: usize) -> $type {
            let mut bs = [0u8; $l];
            bs.copy_from_slice(&src[idx * $l..(idx * $l + $l)]);
            <$type>::from_le_bytes(bs)
        }
    };
}

macro_rules! write_impl {
    ($name: ident, $type: ty, $l: literal) => {
        pub fn $name(src: &mut [u8], idx: usize, v: $type) {
            let bs = v.to_le_bytes();
            let dst = &mut src[idx * $l..(idx * $l + $l)];
            dst.copy_from_slice(&bs);
        }
    };
}


copy_impl!(copy_to_i32s, i32, 4);

read_impl!(read_i32, i32, 4);
read_impl!(read_u32, u32, 4);
read_impl!(read_u16, u16, 2);
read_impl!(read_i16, i16, 2);
write_impl!(write_i32, i32, 4);
write_impl!(write_u32, u32, 4);
write_impl!(write_u16, u16, 2);
write_impl!(write_i16, i16, 2);

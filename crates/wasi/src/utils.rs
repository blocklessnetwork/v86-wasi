
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
    let x: u32 = x as u32;
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
    ($name: ident, $type: ty) => {
        pub fn $name(src: &[u8], dst: &mut [$type]) {
            const SZ: usize = std::mem::size_of::<$type>();
            let mut bs = [0u8; SZ];
            for i in 0..dst.len() {
                let start = i * SZ;
                let end = start + SZ;
                bs.copy_from_slice(&src[start..end]);
                let t: $type = <$type>::from_le_bytes(bs);
                dst[i] = t;
            }
        }
    };
}

macro_rules! read_impl {
    ($name: ident, $type: ty) => {
        pub fn $name(src: &[u8], idx: usize) -> $type {
            const SZ: usize = std::mem::size_of::<$type>();
            let mut bs = [0u8; SZ];
            bs.copy_from_slice(&src[idx * SZ..(idx * SZ + SZ)]);
            <$type>::from_le_bytes(bs)
        }
    };
}

macro_rules! write_impl {
    ($name: ident, $type: ty) => {
        pub fn $name(src: &mut [u8], idx: usize, v: $type) {
            const SZ: usize = std::mem::size_of::<$type>();
            let bs = v.to_le_bytes();
            let dst = &mut src[idx * SZ..(idx * SZ + SZ)];
            dst.copy_from_slice(&bs);
        }
    };
}

copy_impl!(copy_to_i32s, i32);

read_impl!(read_i32, i32);
read_impl!(read_u32, u32);
read_impl!(read_u16, u16);
read_impl!(read_i16, i16);
write_impl!(write_i32, i32);
write_impl!(write_u32, u32);
write_impl!(write_u16, u16);
write_impl!(write_i16, i16);

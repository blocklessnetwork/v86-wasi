
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
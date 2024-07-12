use crate::LOG;


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

pub struct UTF8;

impl UTF8 {
    pub fn utf8_length(s: &str) -> u32 {
        let mut l = 0;
        for c in s.chars() {
            l += if (c as i32) < 128 {
                1
            } else {
                2
            };
        }
        l
    }
}


pub struct UTF8StreamToUnicode {
    stream: [u8; 5],
    ofs: usize,
}

impl UTF8StreamToUnicode {
    pub fn new() -> UTF8StreamToUnicode {
        UTF8StreamToUnicode {
            stream: [0; 5],
            ofs: 0,
        }
    }
    pub fn put(&mut self, key: u8) -> i8 {
        self.stream[self.ofs] = key;
        self.ofs += 1;
        match self.ofs {
            1 => {
                if self.stream[0] < 128 {
                    self.ofs = 0;
                    return self.stream[0] as i8;
                }
            }
            2 => {
                if self.stream[0] & 0xE0 == 0xC0 {
                    if self.stream[1] & 0xC0 == 0x80 {
                        self.ofs = 0;
                        return ((self.stream[0] &0x1F)<<6|(self.stream[1] &0x3F)) as i8
                    }
                }
            }
            _ => {}
        }
        return -1;
    }
}

pub enum Utf8S {
    U8(u8),
    U16(u8, u8),
}

pub fn unicode_to_utf8stream(key: i32) -> Option<Utf8S> {
    if key < 0x80 {
        return Some(Utf8S::U8(key as u8));
    }
    if key < 0x800 {
        return Some(Utf8S::U16(0xC0|(((key>>6) as u8)&0x1F), 0x80|((key&0x3F) as u8)));
    }
    return None;
}

#[derive(Clone, Copy, Debug)]
pub struct Qid {
    pub type_: u8,
    pub version: u32,
    pub path: u32,
}

#[derive(Debug)]
pub enum MarVal  {
    String(String),
    U32(u32),
    U16(u16),
    U8(u8),
    QID(Qid)
}

impl MarVal {
    pub fn as_u32(&self) -> Option<u32> {
        match *self {
            MarVal::U32(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match *self {
            MarVal::U16(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match *self {
            MarVal::U8(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            MarVal::String(ref u) => Some(u),
            _ => None,
        }
    }

    pub fn as_qid(&self) -> Option<Qid> {
        match *self {
            MarVal::QID(u) => Some(u),
            _ => None,
        }
    }
}

pub struct State {
    pub offset: usize,
}

pub struct Marshall;

impl Marshall {
    pub fn marshall(typelist: &[&str], input: &[MarVal], mut struct_: BufferHodler) -> u32 {
        let mut size = 0;
        for i in 0..typelist.len() {
            let item = &input[i];
            match typelist[i] {
                "w" => {
                    let item = item.as_u32().unwrap();
                    struct_.push((item&0xFF) as u8);
                    struct_.push(((item >> 8)&0xFF) as u8);
                    struct_.push(((item >> 16)&0xFF) as u8);
                    struct_.push(((item >> 24)&0xFF) as u8);
                    size += 4;
                }
                "d" => {
                    let item = item.as_u32().unwrap();
                    struct_.push((item&0xFF) as u8);
                    struct_.push(((item >> 8)&0xFF) as u8);
                    struct_.push(((item >> 16)&0xFF) as u8);
                    struct_.push(((item >> 24)&0xFF) as u8);
                    struct_.push(0);
                    struct_.push(0);
                    struct_.push(0);
                    struct_.push(0);
                    size += 8;
                }
                "h" => {
                    let item = item.as_u16().unwrap();
                    struct_.push((item&0xFF) as u8);
                    struct_.push(((item >> 8)&0xFF) as u8);
                    size += 2;
                }
                "b" => {
                    let item = item.as_u8().unwrap();
                    struct_.push((item&0xFF) as u8);
                    size += 1;
                }
                "s" => {
                    let lengthoffset = struct_.offset;
                    struct_.push(0);
                    struct_.push(0);
                    size += 2;
                    let s = item.as_str().unwrap();
                    let mut slen: u32 = 0;
                    for c in s.chars() {
                        let utf8 = unicode_to_utf8stream(c as _);
                        match utf8 {
                            Some(Utf8S::U16(u1, u2)) => {
                                struct_.push(u1);
                                struct_.push(u2);
                                slen += 2;
                                size += 2;
                            }
                            Some(Utf8S::U8(u)) => {
                                struct_.push(u);
                                slen += 1;
                                size += 1;
                            }
                            _ => {}
                        }
                    }
                    struct_.put(lengthoffset, (slen & 0xFF) as u8);
                    struct_.put(lengthoffset+1, ((slen >> 8) & 0xFF) as u8);

                }
                "Q" => {
                    let qid = item.as_qid().unwrap();
                    let input = &[MarVal::U8(qid.type_), MarVal::U32(qid.version), MarVal::U32(qid.path)];
                    Marshall::marshall(&["b", "w", "d"], input, struct_);
                    struct_.offset += 13;
                    size += 13;
                }
                _=> {
                    dbg_log!(LOG::P9,  "Marshall: Unknown type={}", typelist[i]);
                }
            }
        }
        size
    }

    pub fn unmarshall(typelist: &[&str], struct_: &[u8], state: &mut State) ->  Vec<MarVal> {
        let mut offset  = state.offset;
        let mut output = Vec::new();
        for type_ in typelist {
            match *type_ {
                "w" => {
                    let mut val = struct_[offset] as u32;
                    val += (struct_[offset+1] as u32) << 8;
                    val += (struct_[offset+2] as u32) << 16;
                    val += (struct_[offset+3] as u32) << 24;
                    offset += 4;
                    output.push(MarVal::U32(val));
                }
                "d" => {
                    let mut val = struct_[offset] as u32;
                    val += (struct_[offset+1] as u32) << 8;
                    val += (struct_[offset+2] as u32) << 16;
                    val += (struct_[offset+3] as u32) << 24;
                    offset += 4;
                    offset += 4;
                    output.push(MarVal::U32(val));
                }
                "h" => {
                    let mut val = struct_[offset] as u16;
                    val += (struct_[offset+1] as u16) << 8;
                    offset += 2;
                    output.push(MarVal::U16(val));
                }
                "b" => {
                    let val = struct_[offset];
                    offset += 1;
                    output.push(MarVal::U8(val));
                }
                "s" => {
                    let mut len = struct_[offset] as u16;
                    len += (struct_[offset + 1] as u16) << 8;
                    offset += 2;
                    let mut s = String::new();
                    let mut utf8converter = UTF8StreamToUnicode::new();
                    for _ in 0..len {
                        let c = utf8converter.put(struct_[offset]);
                        offset += 1;
                        if c == -1 {
                            continue;
                        }
                        s.push(c as u8 as char);
                    }
                    output.push(MarVal::String(s));
                }
                "Q" => {
                    state.offset = offset;
                    let res = Marshall::unmarshall(&["b", "w", "d"], struct_, state);
                    let qid = Qid {
                        type_: res[0].as_u8().unwrap(),
                        version: res[1].as_u32().unwrap(),
                        path: res[2].as_u32().unwrap(),
                    };
                    output.push(MarVal::QID ( qid ))
                }
                _ => {
                    dbg_log!(LOG::P9,  "Error in Unmarshall: Unknown type={}", type_);
                }
            }
        }
        state.offset = offset;
        output
    }
}



#[derive(Clone, Copy)]
pub struct BufferHodler {
    offset: usize,
    buf: *mut Vec<u8>
}


impl BufferHodler {
    #[inline(always)]
    pub fn new(buf :&mut Vec<u8>, offset: usize) -> Self {
        Self {
            buf: buf as _,
            offset,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, val: u8) {
        let index: usize = self.offset;
        unsafe {
            if index >= (*self.buf).len() {
                (*self.buf).push(val);
            } else {
                (*self.buf)[index] = val;
            }
        }
        self.offset += 1;
    }

    #[inline(always)]
    pub fn put(&mut self, off: usize, val: u8) {
        unsafe {(*self.buf)[off] = val;}
    }
}



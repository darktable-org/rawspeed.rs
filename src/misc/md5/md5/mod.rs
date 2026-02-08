use crate::svec::SVec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MD5State([u32; 4]);

impl<'a> IntoIterator for &'a MD5State {
    type Item = &'a u32;
    type IntoIter = core::slice::Iter<'a, u32>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut MD5State {
    type Item = &'a mut u32;
    type IntoIter = core::slice::IterMut<'a, u32>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl From<MD5State> for String {
    #[inline(never)]
    fn from(val: MD5State) -> Self {
        let mut str = String::with_capacity(2 * (4 * 32) / 8);
        assert_eq!(str.capacity(), 32);
        for b in val.iter().flat_map(|&e| e.to_le_bytes()) {
            use core::fmt::Write as _;
            write!(str, "{b:0>2x}").unwrap();
        }
        assert_eq!(str.len(), 32);
        str
    }
}

#[derive(Debug, Clone, Copy)]
struct MD5Block([u8; 64]);

struct MD5Round(usize);

struct StepParams {
    k: usize,
    s: u32,
    t: u32,
}
impl StepParams {
    #[inline]
    #[must_use]
    const fn new(k: usize, s: u32, t: u32) -> Self {
        Self { k, s, t }
    }
}
const STAGES: [[StepParams; 16]; 4] = [
    [
        StepParams::new(0, 7, 0xD76A_A478),
        StepParams::new(1, 12, 0xE8C7_B756),
        StepParams::new(2, 17, 0x2420_70DB),
        StepParams::new(3, 22, 0xC1BD_CEEE),
        StepParams::new(4, 7, 0xF57C_0FAF),
        StepParams::new(5, 12, 0x4787_C62A),
        StepParams::new(6, 17, 0xA830_4613),
        StepParams::new(7, 22, 0xFD46_9501),
        StepParams::new(8, 7, 0x6980_98D8),
        StepParams::new(9, 12, 0x8B44_F7AF),
        StepParams::new(10, 17, 0xFFFF_5BB1),
        StepParams::new(11, 22, 0x895C_D7BE),
        StepParams::new(12, 7, 0x6B90_1122),
        StepParams::new(13, 12, 0xFD98_7193),
        StepParams::new(14, 17, 0xA679_438E),
        StepParams::new(15, 22, 0x49B4_0821),
    ],
    [
        StepParams::new(1, 5, 0xF61E_2562),
        StepParams::new(6, 9, 0xC040_B340),
        StepParams::new(11, 14, 0x265E_5A51),
        StepParams::new(0, 20, 0xE9B6_C7AA),
        StepParams::new(5, 5, 0xD62F_105D),
        StepParams::new(10, 9, 0x0244_1453),
        StepParams::new(15, 14, 0xD8A1_E681),
        StepParams::new(4, 20, 0xE7D3_FBC8),
        StepParams::new(9, 5, 0x21E1_CDE6),
        StepParams::new(14, 9, 0xC337_07D6),
        StepParams::new(3, 14, 0xF4D5_0D87),
        StepParams::new(8, 20, 0x455A_14ED),
        StepParams::new(13, 5, 0xA9E3_E905),
        StepParams::new(2, 9, 0xFCEF_A3F8),
        StepParams::new(7, 14, 0x676F_02D9),
        StepParams::new(12, 20, 0x8D2A_4C8A),
    ],
    [
        StepParams::new(5, 4, 0xFFFA_3942),
        StepParams::new(8, 11, 0x8771_F681),
        StepParams::new(11, 16, 0x6D9D_6122),
        StepParams::new(14, 23, 0xFDE5_380C),
        StepParams::new(1, 4, 0xA4BE_EA44),
        StepParams::new(4, 11, 0x4BDE_CFA9),
        StepParams::new(7, 16, 0xF6BB_4B60),
        StepParams::new(10, 23, 0xBEBF_BC70),
        StepParams::new(13, 4, 0x289B_7EC6),
        StepParams::new(0, 11, 0xEAA1_27FA),
        StepParams::new(3, 16, 0xD4EF_3085),
        StepParams::new(6, 23, 0x0488_1D05),
        StepParams::new(9, 4, 0xD9D4_D039),
        StepParams::new(12, 11, 0xE6DB_99E5),
        StepParams::new(15, 16, 0x1FA2_7CF8),
        StepParams::new(2, 23, 0xC4AC_5665),
    ],
    [
        StepParams::new(0, 6, 0xF429_2244),
        StepParams::new(7, 10, 0x432A_FF97),
        StepParams::new(14, 15, 0xAB94_23A7),
        StepParams::new(5, 21, 0xFC93_A039),
        StepParams::new(12, 6, 0x655B_59C3),
        StepParams::new(3, 10, 0x8F0C_CC92),
        StepParams::new(10, 15, 0xFFEF_F47D),
        StepParams::new(1, 21, 0x8584_5DD1),
        StepParams::new(8, 6, 0x6FA8_7E4F),
        StepParams::new(15, 10, 0xFE2C_E6E0),
        StepParams::new(6, 15, 0xA301_4314),
        StepParams::new(13, 21, 0x4E08_11A1),
        StepParams::new(4, 6, 0xF753_7E82),
        StepParams::new(11, 10, 0xBD3A_F235),
        StepParams::new(2, 15, 0x2AD7_D2BB),
        StepParams::new(9, 21, 0xEB86_D391),
    ],
];

impl MD5Round {
    #[inline]
    #[must_use]
    const fn get_expr(self, tmp: MD5State) -> u32 {
        let [_a, b, c, d] = tmp.0;
        match self.0 {
            0 => d ^ (b & (c ^ d)),
            1 => c ^ (d & (b ^ c)),
            2 => b ^ c ^ d,
            3 => c ^ (b | !d),
            _ => unreachable!(),
        }
    }
}

impl MD5State {
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, u32> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, u32> {
        self.0.iter_mut()
    }

    #[inline]
    #[must_use]
    pub const fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        Self([a, b, c, d])
    }

    #[inline]
    #[must_use]
    pub const fn init() -> Self {
        Self::new(0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476)
    }

    #[inline]
    #[expect(clippy::many_single_char_names)]
    fn round<S>(&mut self, r: MD5Round, step: &StepParams, schedule: S)
    where
        S: Fn(usize) -> u32,
    {
        let [a, b, c, d] = self.0;
        let expr = r.get_expr(*self);
        let a = a
            .wrapping_add(expr)
            .wrapping_add(step.t)
            .wrapping_add(schedule(step.k))
            .rotate_left(step.s)
            .wrapping_add(b);
        self.0 = [d, a, b, c];
    }

    #[inline]
    fn md5_compress(&mut self, block: &MD5Block) {
        let schedule = |k: usize| -> u32 {
            let bytes = block.0.chunks_exact(32 / 8).nth(k).unwrap();
            u32::from_le_bytes(bytes.try_into().unwrap())
        };

        let mut tmp = *self;
        for (round, steps) in STAGES.iter().enumerate() {
            for step in steps {
                tmp.round(MD5Round(round), step, schedule);
            }
        }

        for (x, t) in self.iter_mut().zip(tmp.iter()) {
            *x = x.wrapping_add(*t);
        }
    }
}

#[derive(Debug)]
pub struct MD5 {
    buf: SVec<u8, 64>,
    state: MD5State,
    bytes_total: usize,
}

impl MD5 {
    const MAGIC0: [u8; 1] = [0x80];
    const ZERO_PADDING: [u8; 64] = [0_u8; 64];

    #[inline]
    pub fn extend(&mut self, mut msg: &[u8]) {
        assert!(!self.buf.is_full());

        if msg.is_empty() {
            return;
        }

        if !self.buf.is_empty() {
            let prefix_len =
                core::cmp::min(msg.len(), self.buf.remaining_capacity());
            let (prefix_msg, rest) = msg.split_at_checked(prefix_len).unwrap();
            msg = rest;
            self.buf.extend(prefix_msg);
            self.bytes_total += prefix_msg.len();
            if self.buf.is_full() {
                let full_block = MD5Block(self.buf[..].try_into().unwrap());
                self.state.md5_compress(&full_block);
                self.buf = SVec::default();
            }
        }

        if msg.is_empty() {
            return;
        }

        assert!(self.buf.is_empty());

        let mut iter = msg.chunks_exact(64);
        for full_block in iter.by_ref() {
            self.bytes_total += full_block.len();
            let full_block = MD5Block(full_block.try_into().unwrap());
            self.state.md5_compress(&full_block);
        }

        msg = iter.remainder();
        self.buf.extend(msg);
        self.bytes_total += msg.len();
        assert!(!self.buf.is_full());
    }

    #[inline]
    #[must_use]
    pub fn flush(mut self) -> MD5State {
        assert!(!self.buf.is_full());

        self.buf.extend(&Self::MAGIC0);

        while self.buf.remaining_capacity() != 8 {
            let num = if self.buf.remaining_capacity() > 8 {
                self.buf.remaining_capacity() - 8
            } else {
                self.buf.remaining_capacity()
            };
            self.buf.extend(Self::ZERO_PADDING.get(..num).unwrap());
            if self.buf.is_full() {
                let full_block = MD5Block(self.buf[..].try_into().unwrap());
                self.state.md5_compress(&full_block);
                self.buf = SVec::default();
            }
        }

        let bits_total = u64::try_from(self.bytes_total)
            .unwrap()
            .checked_mul(8)
            .unwrap();
        let magic1: [u8; 8] = bits_total.to_le_bytes();
        self.buf.extend(&magic1);
        let full_block = MD5Block(self.buf[..].try_into().unwrap());
        self.state.md5_compress(&full_block);
        self.state
    }
}

impl Default for MD5 {
    #[inline]
    fn default() -> Self {
        Self {
            buf: SVec::default(),
            state: MD5State::init(),
            bytes_total: 0,
        }
    }
}

#[cfg(test)]
mod tests;

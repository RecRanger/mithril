use std::fmt;

// ---- AES S-boxes ----

const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

// ---- Precomputed T-tables ----

const fn xtime(a: u8) -> u8 {
    if a & 0x80 != 0 {
        (a << 1) ^ 0x1b
    } else {
        a << 1
    }
}

const fn make_enc_tables() -> ([u32; 256], [u32; 256], [u32; 256], [u32; 256]) {
    let mut t0 = [0u32; 256];
    let mut t1 = [0u32; 256];
    let mut t2 = [0u32; 256];
    let mut t3 = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = SBOX[i];
        let x2 = xtime(s);
        let x3 = x2 ^ s;
        t0[i] = (x2 as u32) | ((s as u32) << 8) | ((s as u32) << 16) | ((x3 as u32) << 24);
        t1[i] = (x3 as u32) | ((x2 as u32) << 8) | ((s as u32) << 16) | ((s as u32) << 24);
        t2[i] = (s as u32) | ((x3 as u32) << 8) | ((x2 as u32) << 16) | ((s as u32) << 24);
        t3[i] = (s as u32) | ((s as u32) << 8) | ((x3 as u32) << 16) | ((x2 as u32) << 24);
        i += 1;
    }
    (t0, t1, t2, t3)
}

const fn make_dec_tables() -> ([u32; 256], [u32; 256], [u32; 256], [u32; 256]) {
    let mut t0 = [0u32; 256];
    let mut t1 = [0u32; 256];
    let mut t2 = [0u32; 256];
    let mut t3 = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = INV_SBOX[i];
        let x2 = xtime(s);
        let x4 = xtime(x2);
        let x8 = xtime(x4);
        let x9 = x8 ^ s;
        let xb = x8 ^ x2 ^ s;
        let xd = x8 ^ x4 ^ s;
        let xe = x8 ^ x4 ^ x2;
        t0[i] = (xe as u32) | ((x9 as u32) << 8) | ((xd as u32) << 16) | ((xb as u32) << 24);
        t1[i] = (xb as u32) | ((xe as u32) << 8) | ((x9 as u32) << 16) | ((xd as u32) << 24);
        t2[i] = (xd as u32) | ((xb as u32) << 8) | ((xe as u32) << 16) | ((x9 as u32) << 24);
        t3[i] = (x9 as u32) | ((xd as u32) << 8) | ((xb as u32) << 16) | ((xe as u32) << 24);
        i += 1;
    }
    (t0, t1, t2, t3)
}

static ENC_TABLE: ([u32; 256], [u32; 256], [u32; 256], [u32; 256]) = make_enc_tables();
static DEC_TABLE: ([u32; 256], [u32; 256], [u32; 256], [u32; 256]) = make_dec_tables();

// ---- AES round functions ----

fn aesenc(state: u128, key: u128) -> u128 {
    let b = state.to_le_bytes();
    let mut out = [0u32; 4];
    for c in 0..4usize {
        let b0 = b[((c) % 4) * 4] as usize;
        let b1 = b[((c + 1) % 4) * 4 + 1] as usize;
        let b2 = b[((c + 2) % 4) * 4 + 2] as usize;
        let b3 = b[((c + 3) % 4) * 4 + 3] as usize;
        out[c] = ENC_TABLE.0[b0] ^ ENC_TABLE.1[b1] ^ ENC_TABLE.2[b2] ^ ENC_TABLE.3[b3];
    }
    let mut result = [0u8; 16];
    for c in 0..4 {
        result[c * 4..c * 4 + 4].copy_from_slice(&out[c].to_le_bytes());
    }
    u128::from_le_bytes(result) ^ key
}

fn aesdec(state: u128, key: u128) -> u128 {
    let b = state.to_le_bytes();
    let mut out = [0u32; 4];
    for c in 0..4usize {
        let b0 = b[((c) % 4) * 4] as usize;
        let b1 = b[((c + 3) % 4) * 4 + 1] as usize;
        let b2 = b[((c + 2) % 4) * 4 + 2] as usize;
        let b3 = b[((c + 1) % 4) * 4 + 3] as usize;
        out[c] = DEC_TABLE.0[b0] ^ DEC_TABLE.1[b1] ^ DEC_TABLE.2[b2] ^ DEC_TABLE.3[b3];
    }
    let mut result = [0u8; 16];
    for c in 0..4 {
        result[c * 4..c * 4 + 4].copy_from_slice(&out[c].to_le_bytes());
    }
    u128::from_le_bytes(result) ^ key
}

// ---- m128i ----

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[allow(nonstandard_style)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct m128i(pub u128);

// ---- aarch64 helpers ----

#[cfg(target_arch = "aarch64")]
unsafe fn u128_to_uint8x16(v: u128) -> uint8x16_t {
    let bytes = v.to_le_bytes();
    vld1q_u8(bytes.as_ptr())
}

#[cfg(target_arch = "aarch64")]
unsafe fn uint8x16_to_u128(v: uint8x16_t) -> u128 {
    let mut bytes = [0u8; 16];
    vst1q_u8(bytes.as_mut_ptr(), v);
    u128::from_le_bytes(bytes)
}

impl m128i {
    pub fn aesenc(&self, key: m128i) -> m128i {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("aes") {
                return unsafe { self.hw_aesenc_x86(key) };
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            if std::arch::is_aarch64_feature_detected!("aes") {
                return unsafe { self.hw_aesenc_aarch64(key) };
            }
        }
        self.sw_aesenc(key)
    }

    pub fn aesdec(&self, key: m128i) -> m128i {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("aes") {
                return unsafe { self.hw_aesdec_x86(key) };
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            if std::arch::is_aarch64_feature_detected!("aes") {
                return unsafe { self.hw_aesdec_aarch64(key) };
            }
        }
        self.sw_aesdec(key)
    }

    // --- x86 Hardware ---

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "aes")]
    unsafe fn hw_aesenc_x86(&self, key: m128i) -> m128i {
        unsafe {
            let s = _mm_loadu_si128(&self.0 as *const u128 as *const __m128i);
            let k = _mm_loadu_si128(&key.0 as *const u128 as *const __m128i);
            let r = _mm_aesenc_si128(s, k);
            let mut out = 0u128;
            _mm_storeu_si128(&mut out as *mut u128 as *mut __m128i, r);
            m128i(out)
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "aes")]
    unsafe fn hw_aesdec_x86(&self, key: m128i) -> m128i {
        unsafe {
            let s = _mm_loadu_si128(&self.0 as *const u128 as *const __m128i);
            let k = _mm_loadu_si128(&key.0 as *const u128 as *const __m128i);
            let r = _mm_aesdec_si128(s, k);
            let mut out = 0u128;
            _mm_storeu_si128(&mut out as *mut u128 as *mut __m128i, r);
            m128i(out)
        }
    }

    // --- aarch64 Hardware ---

    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "aes")]
    unsafe fn hw_aesenc_aarch64(&self, key: m128i) -> m128i {
        unsafe {
            let s = u128_to_uint8x16(self.0);
            let k = u128_to_uint8x16(key.0);
            let zero = vdupq_n_u8(0);
            let mut out = vaeseq_u8(s, zero);
            out = vaesmcq_u8(out);
            let res = veorq_u8(out, k);
            m128i(uint8x16_to_u128(res))
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "aes")]
    unsafe fn hw_aesdec_aarch64(&self, key: m128i) -> m128i {
        unsafe {
            let s = u128_to_uint8x16(self.0);
            let k = u128_to_uint8x16(key.0);
            let zero = vdupq_n_u8(0);
            let mut out = vaesdq_u8(s, zero);
            out = vaesimcq_u8(out);
            let res = veorq_u8(out, k);
            m128i(uint8x16_to_u128(res))
        }
    }

    // --- Software Fallback ---

    fn sw_aesenc(&self, key: m128i) -> m128i {
        m128i(aesenc(self.0, key.0))
    }

    fn sw_aesdec(&self, key: m128i) -> m128i {
        m128i(aesdec(self.0, key.0))
    }

    pub fn zero() -> m128i {
        m128i(0)
    }

    pub fn from_u8(bytes: &[u8]) -> m128i {
        debug_assert_eq!(bytes.len(), 16);
        m128i(u128::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn from_i32(i3: i32, i2: i32, i1: i32, i0: i32) -> m128i {
        let v = ((i3 as u128) << 96)
            | (((i2 as u32) as u128) << 64)
            | (((i1 as u32) as u128) << 32)
            | ((i0 as u32) as u128);
        m128i(v)
    }

    pub fn from_u64(u1: u64, u0: u64) -> m128i {
        m128i(((u1 as u128) << 64) | (u0 as u128))
    }

    pub fn as_i64(&self) -> (i64, i64) {
        let lo = self.0 as i64;
        let hi = (self.0 >> 64) as i64;
        (hi, lo)
    }

    pub fn lower_to_m128d(&self) -> m128d {
        let i0 = self.0 as i32 as f64;
        let i1 = (self.0 >> 32) as i32 as f64;
        m128d::from_f64(i1, i0)
    }

    pub fn as_m128d(&self) -> m128d {
        let lo = self.0 as u64;
        let hi = (self.0 >> 64) as u64;
        m128d::from_u64(hi, lo)
    }
}

fn format_m128i(m: &m128i, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (hi, lo) = m.as_i64();
    f.write_fmt(format_args!("({:x},{:x})", hi, lo))
}

impl fmt::LowerHex for m128i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_m128i(self, f)
    }
}

impl fmt::Debug for m128i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_m128i(self, f)
    }
}

// ---- m128d ----

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m128d(pub u128);

impl m128d {
    pub fn zero() -> m128d {
        m128d::from_f64(0.0, 0.0)
    }

    pub fn from_u64(h: u64, l: u64) -> m128d {
        m128d::from_f64(f64::from_bits(h), f64::from_bits(l))
    }

    pub fn from_f64(h: f64, l: f64) -> m128d {
        let lo = l.to_bits() as u128;
        let hi = (h.to_bits() as u128) << 64;
        m128d(hi | lo)
    }

    pub fn as_f64(&self) -> (f64, f64) {
        let lo = f64::from_bits(self.0 as u64);
        let hi = f64::from_bits((self.0 >> 64) as u64);
        (hi, lo)
    }

    pub fn as_u64(&self) -> (u64, u64) {
        let (h, l) = self.as_f64();
        (h.to_bits(), l.to_bits())
    }

    pub fn shuffle_1(&self, other: &m128d) -> m128d {
        let (self_hi, _) = self.as_f64();
        let (_, other_lo) = other.as_f64();
        m128d::from_f64(other_lo, self_hi)
    }

    pub fn sqrt(&self) -> m128d {
        let (h, l) = self.as_f64();
        m128d::from_f64(h.sqrt(), l.sqrt())
    }
}

impl PartialEq for m128d {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for m128d {}

impl std::ops::Add for m128d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let (h1, l1) = self.as_f64();
        let (h2, l2) = other.as_f64();
        m128d::from_f64(h1 + h2, l1 + l2)
    }
}

impl std::ops::Sub for m128d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let (h1, l1) = self.as_f64();
        let (h2, l2) = other.as_f64();
        m128d::from_f64(h1 - h2, l1 - l2)
    }
}

impl std::ops::Mul for m128d {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let (h1, l1) = self.as_f64();
        let (h2, l2) = rhs.as_f64();
        m128d::from_f64(h1 * h2, l1 * l2)
    }
}

impl std::ops::Div for m128d {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let (h1, l1) = self.as_f64();
        let (h2, l2) = rhs.as_f64();
        m128d::from_f64(h1 / h2, l1 / l2)
    }
}

impl std::ops::BitXor for m128d {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        m128d(self.0 ^ rhs.0)
    }
}

impl std::ops::BitAnd for m128d {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        m128d(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for m128d {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        m128d(self.0 | rhs.0)
    }
}

fn format_m128d(m: &m128d, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (hi, lo) = m.as_f64();
    f.write_fmt(format_args!("({},{})", lo, hi))
}

impl fmt::LowerHex for m128d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hi, lo) = self.as_f64();
        f.write_fmt(format_args!("({:x},{:x})", hi.to_bits(), lo.to_bits()))
    }
}

impl fmt::Debug for m128d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_m128d(self, f)
    }
}

// ---- Tests ----

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2",
        target_feature = "aes",
    ))]
    mod hw {
        use std::arch::x86_64::{__m128i, _mm_aesdec_si128, _mm_aesenc_si128, _mm_set_epi64x};

        pub unsafe fn hw_aesenc(state: u128, key: u128) -> u128 {
            let s = u128_to_m128i(state);
            let k = u128_to_m128i(key);
            let r = _mm_aesenc_si128(s, k);
            m128i_to_u128(r)
        }

        pub unsafe fn hw_aesdec(state: u128, key: u128) -> u128 {
            let s = u128_to_m128i(state);
            let k = u128_to_m128i(key);
            let r = _mm_aesdec_si128(s, k);
            m128i_to_u128(r)
        }

        unsafe fn u128_to_m128i(v: u128) -> __m128i {
            let lo = v as i64;
            let hi = (v >> 64) as i64;
            _mm_set_epi64x(hi, lo)
        }

        unsafe fn m128i_to_u128(v: __m128i) -> u128 {
            use std::arch::x86_64::_mm_extract_epi64;
            let lo = _mm_extract_epi64(v, 0) as u64 as u128;
            let hi = (_mm_extract_epi64(v, 1) as u64 as u128) << 64;
            hi | lo
        }
    }

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2",
        target_feature = "aes",
    ))]
    const TEST_VECTORS: &[(u128, u128)] = &[
        (
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
        ),
        (
            0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
        ),
        (
            0x3243f6a8885a308d313198a2e0370734,
            0x2b7e151628aed2a6abf7158809cf4f3c,
        ),
        (
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            0xdead_beef_cafe_babe_1234_5678_9abc_def0,
        ),
        (
            0xdead_beef_cafe_babe_1234_5678_9abc_def0,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
        ),
        (
            0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa,
            0x5555_5555_5555_5555_5555_5555_5555_5555,
        ),
        (
            0x0f0e_0d0c_0b0a_0908_0706_0504_0302_0100,
            0x1f1e_1d1c_1b1a_1918_1716_1514_1312_1110,
        ),
        (
            0x6bc1bee22e409f96e93d7e117393172a,
            0xae2d8a571e03ac9c9eb76fac45af8e51,
        ),
        (
            0x0000_0000_0000_0000_0000_0000_0000_0001,
            0x0000_0000_0000_0000_0000_0000_0000_0000,
        ),
        (
            0x0000_0000_0000_0000_0000_0000_0000_0000,
            0x8000_0000_0000_0000_0000_0000_0000_0000,
        ),
    ];

    #[test]
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2",
        target_feature = "aes",
    ))]
    fn aesenc_matches_hardware() {
        let mut all_pass = true;
        for (i, &(state, key)) in TEST_VECTORS.iter().enumerate() {
            let sw = aesenc(state, key);
            let hw = unsafe { hw::hw_aesenc(state, key) };
            if sw != hw {
                eprintln!(
                    "[aesenc] vector {i} FAIL\n  state={state:032x}\n  key  ={key:032x}\n  sw   ={sw:032x}\n  hw   ={hw:032x}\n  diff ={:032x}",
                    sw ^ hw
                );
                all_pass = false;
            } else {
                eprintln!("[aesenc] vector {i} ok -> {sw:032x}");
            }
        }
        assert!(all_pass, "aesenc mismatches (see stderr)");
    }

    #[test]
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2",
        target_feature = "aes",
    ))]
    fn aesdec_matches_hardware() {
        let mut all_pass = true;
        for (i, &(state, key)) in TEST_VECTORS.iter().enumerate() {
            let sw = aesdec(state, key);
            let hw = unsafe { hw::hw_aesdec(state, key) };
            if sw != hw {
                eprintln!(
                    "[aesdec] vector {i} FAIL\n  state={state:032x}\n  key  ={key:032x}\n  sw   ={sw:032x}\n  hw   ={hw:032x}\n  diff ={:032x}",
                    sw ^ hw
                );
                all_pass = false;
            } else {
                eprintln!("[aesdec] vector {i} ok -> {sw:032x}");
            }
        }
        assert!(all_pass, "aesdec mismatches (see stderr)");
    }

    #[test]
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2",
        target_feature = "aes",
    ))]
    fn enc_dec_consistency() {
        for (i, &(state, key)) in TEST_VECTORS.iter().enumerate() {
            let sw_enc = aesenc(state, key);
            let sw_dec = aesdec(sw_enc, key);
            let hw_enc = unsafe { hw::hw_aesenc(state, key) };
            let hw_dec = unsafe { hw::hw_aesdec(hw_enc, key) };
            assert_eq!(sw_dec, hw_dec, "vector {i} enc_dec mismatch");
        }
    }

    #[test]
    fn aesenc_perf_smoke() {
        let start = std::time::Instant::now();
        let mut state = 0x3243f6a8885a308d313198a2e0370734u128;
        let key = 0x2b7e151628aed2a6abf7158809cf4f3cu128;
        for _ in 0..1_000_000 {
            state = aesenc(state, key);
        }
        let elapsed = start.elapsed();
        eprintln!(
            "1M aesenc: {:?}  ({} ns/op)",
            elapsed,
            elapsed.as_nanos() / 1_000_000
        );
        let _ = state;
    }
}

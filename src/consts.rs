pub const MAX_COPY: usize = 32;
pub const MAX_LEN: usize = 264; // 256 + 8
pub const MAX_L1_DISTANCE: usize = 8192; // 2^3
pub const MAX_L2_DISTANCE: usize = 8191; // 2^3 - 1
pub const MAX_FARDISTANCE: usize = 65535 + MAX_L2_DISTANCE - 1;

pub const HASH_LOG: usize = 14;
pub const HASH_SIZE: usize = 1 << HASH_LOG;
pub const HASH_MASK: u16 = (HASH_SIZE - 1) as u16;

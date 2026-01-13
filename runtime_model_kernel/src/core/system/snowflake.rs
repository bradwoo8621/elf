use crate::{IdGenerator, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

// const TIMESTAMP_BITS: u8 = 43;
const NODE_ID_BITS: u8 = 12;
const SEQUENCE_BITS: u8 = 12;

const MAX_NODE_ID: u64 = (1 << NODE_ID_BITS) - 1;
const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;

const NODE_SHIFT: u8 = SEQUENCE_BITS;
const TIMESTAMP_SHIFT: u8 = SEQUENCE_BITS + NODE_ID_BITS;

/// 2014-12-31 16:00:00, copy from python version
const EPOCH_MS: u128 = 1420041600000;

/// max value is [99_999_999_999_998_951_423], and in radix 2, which is [2^66 - 1].
/// - the max value of radix 10, is [99_999_999_999_999_999_999],
///   in radix 2, which is 67 bits, is [10_10110_10111_10001_11010_11110_00101_10101_10001_10000_11111_11111_11111_11111]
/// - 12 bits for sequence, max value is [4095],
/// - 12 bits for node, max value is [4095],
/// - 43 bits for timestamp, max value is [5_960_464_477_539], stands for 189 years,
///   in radix 2, which is [101_01101_01111_00011_10101_11100_01011_01011_00011],
/// so the parts should be as below:
/// ```text
/// 101_01101_01111_00011_10101_11100_01011_01011_00011__000_011_111_111__111_111_111_111
/// |---------------------timestamp--------------------||-----node------||---sequence---|
/// ```
/// and since the max value of node and sequence parts are 4095,
/// the last 25 bits changes from [1__000_011_111_111__111_111_111_111] to [0__111_111_111_111__111_111_111_111],
/// so the actual max value will be
/// ```text
/// 101_01101_01111_00011_10101_11100_01011_01011_00010__111_111_111_111__111_111_111_111
/// |---------------------timestamp--------------------||-----node------||---sequence---|
///
///
/// ```
/// which is [99_999_999_999_998_951_423].
/// and the max value of timestamp part is [101_01101_01111_00011_10101_11100_01011_01011_00010],
/// in radix 10, is [5_960_464_477_538], stands for 189 years, OK with this.
pub struct SnowflakeIdGenerator {
    node_id: u64,
    last_timestamp: Mutex<u128>,
    sequence: Mutex<u64>,
}

impl SnowflakeIdGenerator {
    pub fn new(node_id: u64) -> StdR<Self> {
        if node_id > MAX_NODE_ID {
            RuntimeModelKernelErrorCode::SnowflakeNodeIdTooBig.msg(format!(
                "Snowflake id generator node id must be <= {}.",
                MAX_NODE_ID
            ))
        } else {
            Ok(Self {
                node_id,
                last_timestamp: Mutex::new(0),
                sequence: Mutex::new(0),
            })
        }
    }

    fn current_millis() -> u128 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch");
        now.as_millis()
    }
}

impl IdGenerator for SnowflakeIdGenerator {
    fn next_id(&self) -> u128 {
        let mut last_ts = self.last_timestamp.lock().unwrap();
        let mut seq = self.sequence.lock().unwrap();

        let ts = SnowflakeIdGenerator::current_millis();

        // handle clock going backwards by waiting until time catches up
        if ts <= *last_ts {
            // when relative timestamp is greater than last timestamp,
            // means all sequences of relative timestamp are consumed already.
            // never mind, just continue consuming the last timestamp's.
            *seq = (*seq + 1) & MAX_SEQUENCE;
            if *seq == 0 {
                // sequence overflow in same millisecond,
                // simply move last timestamp to next millisecond, credit from future.
                *last_ts = *last_ts + 1;
                *seq = 0;
            }
        } else {
            *seq = 0;
            *last_ts = ts;
        }

        let id = ((ts.saturating_sub(EPOCH_MS)) << TIMESTAMP_SHIFT)
            | ((self.node_id as u128) << NODE_SHIFT)
            | (*seq as u128);

        // safety: our bit allocation keeps id well below the max value
        debug_assert!(id <= 99_999_999_999_998_951_423u128);
        id
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDateTime};

    #[test]
    fn test_() {
        let millis: u128 = 0b101_01101_01111_00011_10101_11100_01011_01011_00010;
        let years: f64 = millis as f64 / 1000.0 / 60.0 / 60.0 / 24.0 / 365.0;

        println!("{}, {}", millis, years);

        vec![
            "1010110111111111111111111111111111111111111111111111111111111111111", // 100304170900795686911
            "1010110101111000111010111100010110101100011000011111111111111111111", // 99999999999999999999
            "1010110101111000111010111100010110101100010111111111111111111111111", // 99999999999998951423
            "0110110101111000111010111100010110101100011111111111111111111111111", // 63106511852596625407
            "                                                       111111111111", // 4095 12 bits
            "                                           111111111111            ", // 4095 12 bits
            "                                            11111111111            ", // 2047 11 bits
            "11111111111111111111111111111111111111111111                       ", // 17592186044415 44 bits
            "                                                                   ", //  1764211056050, 56y
            "10101101011110001110101111000101101011000110                       ", // 11920928955078, 378y
            "1010110101111000111010111100010110101100011                        ", //  5960464477539, 189y
            "1010110101111000111010111100010110101100010                        ", //  5960464477538, 189y
            "0111111111111111111111111111111111111111111                        ", //  4398046511103, 139.4y
            "0110110101111000111010111100010110101100011                        ", //  3761441221987, 119y
            "101011010111100011101011110001011010110001                         ", //  2980232238769, 94.5y
        ];

        let specific_date_str = "2020-11-27 12:00:00.000";
        let specific_date =
            NaiveDateTime::parse_from_str(specific_date_str, "%Y-%m-%d %H:%M:%S%.3f").unwrap();

        let timestamp_millis = specific_date.and_utc().timestamp_millis();
        println!("{}, {}", specific_date_str, timestamp_millis);

        let seconds = 1420041600000 / 1000;
        let subsec_millis = (1420041600000i64 % 1000) as u32;
        println!(
            "{:?}",
            DateTime::from_timestamp(seconds, subsec_millis * 1_000_000)
        )
    }
}

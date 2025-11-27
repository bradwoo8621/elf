use crate::RuntimeModelKernelErrorCode;
use std::sync::{Arc, OnceLock, RwLock};
use watchmen_model::{StdErrorCode, StdR, VoidR};

/// max to 20 digits numbers
pub trait IdGenerator: Send + Sync {
    fn next_id(&self) -> u128;
}

struct DummyIdGenerator();

impl IdGenerator for DummyIdGenerator {
    fn next_id(&self) -> u128 {
        panic!("DummyIdGenerator does not support any ID generator")
    }
}

static GLOBAL_ID_GENERATOR: OnceLock<Arc<RwLock<Box<dyn IdGenerator>>>> = OnceLock::new();

/// the default id generator is [DummyIdGenerator], which is panic!
/// must [set] a new id generator to replace the default one before [next_id],
/// to make sure no panic!
pub struct IdGen();

impl IdGen {
    fn init() -> Arc<RwLock<Box<dyn IdGenerator>>> {
        Arc::new(RwLock::new(
            Box::new(DummyIdGenerator()) as Box<dyn IdGenerator>
        ))
    }

    pub fn next_id() -> StdR<u128> {
        let lock = GLOBAL_ID_GENERATOR.get_or_init(IdGen::init).read();
        match lock {
            Ok(guard) => Ok(guard.next_id()),
            Err(e) => RuntimeModelKernelErrorCode::CannotGetIdGenerator.msg(e.to_string()),
        }
    }

    pub fn set(new_generator: Box<dyn IdGenerator>) -> VoidR {
        let old_arc = GLOBAL_ID_GENERATOR.get_or_init(IdGen::init);
        match old_arc.write() {
            Ok(mut guard) => {
                *guard = new_generator;
                Ok(())
            }
            Err(e) => RuntimeModelKernelErrorCode::CannotSetIdGenerator.msg(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{IdGen, IdGenerator, SnowflakeIdGenerator};
    use std::panic;

    #[test]
    fn test_dummy() {
        panic::set_hook(Box::new(|info| {
            println!("Custom panic hook caught panic: {:?}", info);
        }));

        assert!(panic::catch_unwind(|| IdGen::next_id().unwrap()).is_err());
    }

    #[test]
    fn test_snowflake() {
        let generator =
            SnowflakeIdGenerator::new(1).expect("failed to create SnowflakeIdGenerator");
        IdGen::set(Box::new(generator) as Box<dyn IdGenerator>)
            .expect("Snowflake generator set failed");

        let id = IdGen::next_id().expect("failed to get next id by SnowflakeIdGenerator");
        println!("{}", id);
        assert!(id > 1);
    }
}

use crate::{Pipeline, Topic};
use elf_base::StdR;

pub fn ask_query_performance_pipelines(_topics: Vec<Topic>) -> StdR<Vec<Pipeline>> {
    // TODO define all pipeline monitor pipelines
    Ok(Vec::new())
}

pub mod errors;
pub mod executor;
mod stage1;
mod stage2;
mod stage3;

use errors::Errors;

pub use executor::Context;

pub fn generate_tokens(string: &str) -> Result<Vec<stage3::Stage3Token>, Errors> {
    Ok(stage3::compile(&stage2::compile(
        &string
            .lines()
            .map(|x| stage1::compile(x))
            .flatten()
            .collect::<Vec<stage1::Stage1Token>>()
            .iter()
            .collect(),
    )?)?)
}

pub fn generate_tokens_stage2(string: &str) -> Result<Vec<stage2::Stage2Token>, Errors> {
    stage2::compile(
        &string
            .lines()
            .map(|x| stage1::compile(x))
            .flatten()
            .collect::<Vec<stage1::Stage1Token>>()
            .iter()
            .collect(),
    )
}

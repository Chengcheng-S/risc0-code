use methods::METHODS_ELF;
use risc0_zkvm::{default_executor, ExecutorEnv};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    ();

    let iterations: u32 = 1000;

    // Execute the guest code.
    let env = ExecutorEnv::builder().write(&iterations)?.build()?;
    let exec = default_executor();
    exec.execute(env, METHODS_ELF)?;

    Ok(())
}

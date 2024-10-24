mod cli;
mod process;
mod util;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::*;
pub use util::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}

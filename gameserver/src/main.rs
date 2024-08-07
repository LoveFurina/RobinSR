use anyhow::Result;

mod logging;
mod net;
mod util;

use logging::init_tracing;

pub fn info_supremacy() {
    println!("\n这个本地端是免费的，如果你是买来的说明你被骗了，请立刻去退款");
}

#[tokio::main]
async fn main() -> Result<()> {
    info_supremacy();
    init_tracing();
    net::gateway::listen("127.0.0.1", 23301).await?;

    Ok(())
}

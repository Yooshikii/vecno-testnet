use vecno_cli_lib::vecno_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_vecno_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    vecno_cli(options, None).await?;
    Ok(())
}

use entropyx_cli_lib::entropyx_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_entropyx_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    entropyx_cli(options, None).await?;
    Ok(())
}

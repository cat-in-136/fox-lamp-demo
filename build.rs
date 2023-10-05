extern crate types;

use embuild::build::{CfgArgs, LinkArgs};
use ts_rs::TS;
use types::*;

fn main() -> anyhow::Result<()> {
    export!(MessageType);

    export!(Pixel);

    export!(CommandType);
    export!(DataType);

    CfgArgs::output_propagated("ESP_IDF")?;
    LinkArgs::output_propagated("ESP_IDF")?;

    Ok(())
}

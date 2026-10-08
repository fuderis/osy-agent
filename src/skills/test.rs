use crate::prelude::*;

use anylm::Tool;

pub fn tools_list() -> Vec<Tool> {
    vec![Tool::new("hello", "Prints hello.")]
}

#[log()]
pub async fn handle_hello(tx: Sender<Bytes>, _query: ToolQuery<JsonValue>) -> Result<()> {
    tx.send(Event::Answer("Hello, world!".into()))?;
    Ok(())
}

// TO-DO
use crate::errors::Result;
pub fn read_namespace_link(path: &str) -> Result<String> {
    let path = std::fs::read_link(path)?;
    Ok(path.display().to_string())
}

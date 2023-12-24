use std::fs;
use crate::inventory::Inventory;

pub fn read_config(path: &str) -> anyhow::Result<Inventory, Box<dyn std::error::Error>> {
    let data = fs::read(path)?;
    let text = String::from_utf8(data)?;
    let config: Inventory = toml::from_str(&text)?;
    Ok(config)
}

pub fn write_config(config: &Inventory, path: &str) -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let text = toml::to_string(config)?;
    std::fs::write(path, text)?;
    Ok(())
}
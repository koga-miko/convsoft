use conv_common;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    conv_common::json2yaml()?;

    Ok(())
}

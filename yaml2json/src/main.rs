use conv_common;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    conv_common::yaml2json()?;

    Ok(())
}

use conv_common;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_input = conv_common::load_input()?;
    conv_common::json2yaml(&json_input)?;
    Ok(())
}

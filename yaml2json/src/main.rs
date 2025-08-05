use conv_common;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml_input = conv_common::load_input()?;
    conv_common::yaml2json(&yaml_input)?;
   Ok(())
}

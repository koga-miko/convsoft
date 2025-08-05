use std::env;
use std::fs::File;
use std::io::{self, Read};
use serde_yaml::Value;
use serde_json;

pub fn load_input() -> Result<String, Box<dyn std::error::Error>> {
    // コマンドライン引数を取得（ファイルパスがあるか確認）
    let args: Vec<String> = env::args().collect();

    // 入力ソース（ファイルまたは標準入力）からデータを取得
    let mut input = String::new();
    if args.len() > 1 {
        let path = &args[1];
        let mut file = File::open(path)?;
        file.read_to_string(&mut input)?;
    } else {
        io::stdin().read_to_string(&mut input)?;
    }
    Ok(input)
}

pub fn yaml2json(yaml_text: &String) -> Result<String, Box<dyn std::error::Error>> {
    // YAMLのパース
    let yaml_value: Value = serde_yaml::from_str(yaml_text)?;

    // JSONに変換して標準出力へ
    let json_output = serde_json::to_string(&yaml_value)?;
    println!("{}", json_output);

    Ok(json_output)
}

pub fn json2yaml(json_text: &String) -> Result<String, Box<dyn std::error::Error>> {
    // JSONのパース
    let json_value: Value = serde_json::from_str(json_text)?;

    // YAMLに変換して標準出力へ
    let yaml_output = serde_yaml::to_string(&json_value)?;
    print!("{}", yaml_output);

    Ok(yaml_output)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml2json() {
        let yaml_input = "abc: 123";
        let result = yaml2json(&yaml_input.to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"{"abc":123}"#);
    }

    #[test]
    fn test_json2yaml() {
        let json_input = r#"{"abc":123}"#;
        let result = json2yaml(&json_input.to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "abc: 123\n");
    }
}
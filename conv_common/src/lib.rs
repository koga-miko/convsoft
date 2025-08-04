use std::env;
use std::fs::File;
use std::io::{self, Read};
use serde_yaml::Value;
use serde_json;

fn load_input() -> Result<String, Box<dyn std::error::Error>> {
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

pub fn yaml2json() -> Result<(), Box<dyn std::error::Error>> {
    // 入力ソースからデータを取得
    let input = load_input()?;

    // YAMLのパース
    let yaml_value: Value = serde_yaml::from_str(&input)?;

    // JSONに変換して標準出力へ
    let json_output = serde_json::to_string_pretty(&yaml_value)?;
    println!("{}", json_output);

    Ok(())
}

pub fn json2yaml() -> Result<(), Box<dyn std::error::Error>> {
    // 入力ソースからデータを取得
    let input = load_input()?;

    // JSONのパース
    let json_value: Value = serde_json::from_str(&input)?;

    // YAMLに変換して標準出力へ
    let yaml_output = serde_yaml::to_string(&json_value)?;
    println!("{}", yaml_output);

    Ok(())
}

use serde_json::{json, Map, Value};
use std::fs::File;
use std::io::{self, BufRead};

/// 挿入されたキーのパスに基づいて、ネストされたJSONオブジェクトを構築し、
/// 与えられた値を適切な場所に挿入します。
///
/// # 引数
///
/// - `map`: ルートのJSONオブジェクトを表す `serde_json::Map` の可変参照。
/// - `keys`: キーのパスを表すスライス。各要素がネストの深さを表します。
/// - `value`: キーのパスの末端に挿入される `serde_json::Value`。
pub fn insert_nested_value(map: &mut Map<String, Value>, keys: &[&str], value: Value) {
    if keys.len() == 1 {
        map.insert(keys[0].to_string(), value);
    } else {
        let entry = map
            .entry(keys[0].to_string())
            .or_insert_with(|| json!({}))
            .as_object_mut()
            .unwrap();
        insert_nested_value(entry, &keys[1..], value);
    }
}

/// 指定されたファイルパスからsysctl.conf形式の設定ファイルを読み込み、
/// 設定項目をネストされたJSONオブジェクトとして返します。
///
/// # 引数
///
/// - `filename`: 読み込む設定ファイルのパスを指定する文字列スライス。
///
/// # 戻り値
///
/// - `io::Result<Map<String, Value>>`: 成功した場合は、設定項目が格納された`serde_json::Map`を返します。
///   ファイル読み込みや解析に失敗した場合は、`io::Error`を返します。
pub fn load_config(filename: &str) -> io::Result<Map<String, Value>> {
    let mut map = Map::new();

    if let Ok(file) = File::open(filename) {
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let trimmed = line.trim();

            // コメント行や空行をスキップ
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // "key = value"の形式を解析
            if let Some((key, value)) = trimmed.split_once('=') {
                let key_parts: Vec<&str> = key.trim().split('.').collect();
                let value = value.trim();

                // 真偽値や数値として解釈
                let value_json = if let Ok(boolean) = value.parse::<bool>() {
                    json!(boolean)
                } else if let Ok(number) = value.parse::<f64>() {
                    json!(number)
                } else {
                    json!(value)
                };

                insert_nested_value(&mut map, &key_parts, value_json);
            }
        }
    }

    Ok(map)
}

// 単体テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_nested_value() {
        let mut map = Map::new();
        insert_nested_value(&mut map, &["log", "file"], json!("/var/log/console.log"));
        insert_nested_value(&mut map, &["debug"], json!(true));

        assert_eq!(
            map.get("log").unwrap().get("file").unwrap(),
            "/var/log/console.log"
        );
        assert_eq!(map.get("debug").unwrap(), &json!(true));
    }

    #[test]
    fn test_load_config() {
        let config_str = r#"
            endpoint = localhost:3000
            debug = true
            log.file = /var/log/console.log
        "#;

        // 一時的なファイルを作成してテストに利用
        use std::io::Write;
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", config_str).unwrap();

        let config = load_config(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(config.get("endpoint").unwrap(), "localhost:3000");
        assert_eq!(config.get("debug").unwrap(), &json!(true));
        assert_eq!(
            config.get("log").unwrap().get("file").unwrap(),
            "/var/log/console.log"
        );
    }
}

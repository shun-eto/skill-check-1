use config_loader::load_config;

mod config_loader;

fn main() {
    let config_path = "src/assets/sysctl.conf"; // 読み込むファイルのパスを指定
    match load_config(config_path) {
        Ok(config) => {
            // JSONとして整形して出力
            let json_output = serde_json::to_string_pretty(&config).unwrap();
            println!("{}", json_output);
        }
        Err(e) => eprintln!("Error loading config file: {}", e),
    }
}

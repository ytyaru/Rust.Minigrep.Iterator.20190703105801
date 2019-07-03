fn main() {
    let config = minigrep::Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("異常終了します。引数解析に問題が生じました。: {}", err);
        std::process::exit(1);
    });
    /*
    let args: Vec<String> = std::env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("異常終了します。引数解析に問題が生じました。: {}", err);
        std::process::exit(1);
    });
    */
    println!("query: {}", config.query);
    println!("filename: {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("アプリエラー: {}", e);
        std::process::exit(1);
    };
}


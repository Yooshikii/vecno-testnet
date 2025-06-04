cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn main() {}
    } else {
        use vecno_cli_lib::{vecno_cli, TerminalOptions};

        #[tokio::main]
        async fn main() {
            let result = vecno_cli(TerminalOptions::new().with_prompt("$ "), None).await;
            if let Err(err) = result {
                println!("{err}");
            }
        }
    }
}

use std::env;

fn main() {
    for key in ["WIFI_SSID", "WIFI_PASSWORD"] {
        println!("cargo:rerun-if-env-changed={key}");
        if env::var(key).is_err() {
            panic!("{key} must be set when building the Wi-Fi proof");
        }
    }
}

use std::env;

fn main() {
    for key in [
        "WIFI_SSID",
        "WIFI_PASSWORD",
        "CHATGPT_ACCESS_TOKEN",
        "CHATGPT_ACCOUNT_ID",
    ] {
        println!("cargo:rerun-if-env-changed={key}");
        if env::var(key).is_err() {
            panic!("{key} must be set when building the firmware");
        }
    }

    // These values are intentionally supplied only to the local firmware build.
    // They are not written to the repository, but are embedded in the test image.
    for key in [
        "WIFI_SSID",
        "WIFI_PASSWORD",
        "CHATGPT_ACCESS_TOKEN",
        "CHATGPT_ACCOUNT_ID",
    ] {
        println!("cargo:rustc-env={key}={}", env::var(key).unwrap());
    }
}

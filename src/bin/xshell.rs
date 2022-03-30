use std::os::windows::process::CommandExt;
use std::process::Command;

use url::Url;

fn main() {
    let url_str = get_url_str().expect("Missing URL option.");
    let url = Url::parse(url_str.as_str()).expect("Failed to parse URL string.");
    assert!(url.scheme() == "ssh", "URL protocol should be SSH.");
    let moba_flag = "-newtab";
    Command::new("MobaXterm")
        .args([
            moba_flag,
            format!(
                "sshpass -p {} ssh {}@{} -p {}",
                percent_encoding::percent_decode_str(
                    url.password().expect("URL should include password.")
                )
                .decode_utf8()
                .expect("Failed to decode password."),
                url.username(),
                url.host_str().expect("URL should include host."),
                url.port().unwrap_or(22)
            )
            .as_str(),
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn()
        .expect("Failed to start MobaXterm.");
}

#[inline]
fn get_url_str() -> Option<String> {
    let mut flag_url = false;
    for arg in std::env::args() {
        if flag_url {
            return Some(arg);
        } else {
            flag_url = arg == "-url";
        }
    }
    return None;
}

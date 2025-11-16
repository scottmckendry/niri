use std::fs;

use niri_config::Config;

#[test]
fn included_layout_border_empty_enables_border() {
    let mut dir = std::env::temp_dir();
    dir.push(format!(
        "niri-config-test-{}-{}",
        std::process::id(),
        rand_suffix()
    ));
    fs::create_dir_all(&dir).unwrap();

    let include_path = dir.join("include.kdl");
    fs::write(&include_path, "layout { border {}; }").unwrap();

    let main_path = dir.join("config.kdl");
    fs::write(&main_path, "include \"include.kdl\"").unwrap();

    let config = Config::parse(&main_path, &fs::read_to_string(&main_path).unwrap())
        .config
        .unwrap();

    assert!(
        !config.layout.border.off,
        "border should be enabled via included file"
    );
}

fn rand_suffix() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

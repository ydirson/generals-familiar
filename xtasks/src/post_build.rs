pub fn main() {
    let staging_dir: std::path::PathBuf = std::env::var("TRUNK_STAGING_DIR")
        .expect("TRUNK_STAGING_DIR should be set").into();
    let data_dir = staging_dir.join("data");

    if let Err(e) = opr_test_data::import_data(&data_dir) {
        eprintln!("error: {e:?}");
    }
}

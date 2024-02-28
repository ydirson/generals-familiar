pub fn main() {
    cfg_if::cfg_if! {
        if #[cfg(feature = "local-files")] {
            // FIXME: we ought to use OUT_DIR, but then we have to
            // tell trunk where to find the data
            // See https://github.com/rust-lang/cargo/issues/7546

            //let out_dir: std::path::PathBuf = std::env::var("OUT_DIR")
            //    .expect("OUT_DIR should be set").into();
            //let data_dir = out_dir.join("data");

            let root_dir: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR should be set").into();
            let data_dir = root_dir.join("data");

            if let Err(e) = opr_test_data::import_data(&data_dir) {
                eprintln!("error: {e:?}");
            }
        }
    }
}

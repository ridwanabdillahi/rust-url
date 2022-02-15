use rustc_version::{version_meta, Channel};

fn main() {
    // Set the `debugger_visualizer` cfg flag if the nightly channel is being used.
    if Channel::Nightly == version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=debugger_visualizer");
    }
}

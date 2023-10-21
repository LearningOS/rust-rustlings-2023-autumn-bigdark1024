//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
      let your_command = format!(
        "rustc-env=TEST_FOO={}",
        timestamp
    );
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    let your_command = "cargo:rustc-cfg=feature=\"pass\"";
    println!("{}", your_command);
}

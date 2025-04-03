//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
fn main() { 
    let timestamp = std::time::SystemTime::now() 
       .duration_since(std::time::UNIX_EPOCH) 
       .unwrap() 
       .as_secs(); 
    println!("cargo:rustc-env=TEST_FOO={}", timestamp); 

    //let your_command = "Your command here, please checkout exercises/tests/build.rs";
    println!("cargo:rustc-cfg=feature=\"pass\"");
} 
// Native build script executed before compiling the Rust binary
fn main() {
    println!("cargo:rerun-if-changed=vct_manifest.yml");
    println!("cargo:rerun-if-changed=src/std_lib.vct");
    
    // Links native GPU drivers or dynamic libraries when building target binaries
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=dxgi");
    
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=Metal");
}

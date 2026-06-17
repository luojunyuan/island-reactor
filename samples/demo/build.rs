fn main() {
    islands_reactor_setup::embed_manifest();
    println!("cargo:rerun-if-changed=build.rs");
}

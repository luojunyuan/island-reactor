fn main() {
    island_reactor_setup::embed_example_manifest();
    println!("cargo:rerun-if-changed=build.rs");
}

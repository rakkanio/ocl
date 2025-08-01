use risc0_build::embed_methods;

fn main() {
    // Only embed methods if the guest binaries exist
    if std::path::Path::new("methods/src/bin").exists() {
        embed_methods();
    }
}
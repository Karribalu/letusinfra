use std::env::current_dir;

fn main() {
    // Compile the provider.proto to generate gRPC client code
    println!("cargo:rerun-if-changed=proto/provider.proto");
    println!("{:?}", current_dir());
    let curr_path = current_dir().unwrap();
   println!(" hello {:?}",  curr_path.join("../proto/provider.proto"));
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["../../proto/provider.proto"], &["../../proto"]) // proto include path
        .expect("Failed to compile proto files");
}

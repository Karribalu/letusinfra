fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "../../proto/provider.proto";
    println!("cargo:rerun-if-changed={}", proto_file);
    println!("cargo:rerun-if-changed=../../proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .compile_protos(&[proto_file], &["../../proto"]) ?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate code in build
    // tonic_build::compile_protos("proto/helloworld.proto")?;

    // generate code on own code base, so IDE can reference to
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();

    Ok(())
}

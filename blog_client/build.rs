fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:return-if-changet=build.rs");
    println!("cargo:return-if-changet=proto/blog.proto");

    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .build_client(true)
        .compile_protos(&["proto/blog.proto"], &["proto"])?;
    Ok(())
}

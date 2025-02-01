fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/administration.proto")?;
    tonic_build::compile_protos("proto/game_data.proto")?;
    tonic_build::compile_protos("proto/user.proto")?;
    Ok(())
}

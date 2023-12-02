fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/friends.proto")?;
    tonic_build::compile_protos("proto/users.proto")?;
    tonic_build::compile_protos("proto/misc.proto")?;
    Ok(())
}

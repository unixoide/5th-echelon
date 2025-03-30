pub mod friends {
    tonic::include_proto!("friends"); // The string specified here must match the proto package name
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("friends_descriptor");
}
pub mod users {
    tonic::include_proto!("users"); // The string specified here must match the proto package name
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("users_descriptor");
}
pub mod misc {
    tonic::include_proto!("misc"); // The string specified here must match the proto package name
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("misc_descriptor");
}
pub mod games {
    tonic::include_proto!("games"); // The string specified here must match the proto package name
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("games_descriptor");
}

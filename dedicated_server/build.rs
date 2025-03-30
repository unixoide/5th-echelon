pub fn main() {
    #[cfg(target_os = "windows")]
    winres::WindowsResource::new().compile().unwrap();
}

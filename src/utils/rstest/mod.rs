use rawspeed_utils_librstest::logger;
use rawspeed_utils_librstest::vfs;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let camerasxml_path =
        rawspeed_utils_librstest::rstest::get_camerasxml_path();

    let camerasxml_contents = std::fs::read_to_string(camerasxml_path)
        .expect("Should have been able to read the `cameras.xml");

    let mut log = logger::StdoutLogger::new();
    let mut fs = vfs::NativeFileSystem::new();

    rawspeed_utils_librstest::rstest::main(
        &mut log,
        &mut fs,
        &mut std::env::args(),
        &camerasxml_contents,
    )
}

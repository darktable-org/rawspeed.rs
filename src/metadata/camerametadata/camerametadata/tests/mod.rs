use super::DecodeableCamera;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::Supported;

#[test]
fn supported_as_supported_test() {
    DecodeableCamera::new_if_supported(Supported::Supported).unwrap();
}

#[test]
fn supported_as_unless_unsupported_test() {
    DecodeableCamera::new_unless_unsupported(Supported::Supported).unwrap();
}

#[test]
fn supported_but_no_samples_as_supported_test() {
    DecodeableCamera::new_if_supported(Supported::SupportedNoSamples).unwrap();
}

#[test]
fn supported_but_no_samples_as_unless_unsupported_test() {
    DecodeableCamera::new_unless_unsupported(Supported::SupportedNoSamples)
        .unwrap();
}

#[test]
fn unsupported_as_supported_test() {
    assert_eq!(
        "This camera is not supported",
        DecodeableCamera::new_if_supported(Supported::Unsupported).unwrap_err()
    );
}

#[test]
fn unsupported_as_unless_unsupported_test() {
    assert_eq!(
        "This camera is not supported (explicit)",
        DecodeableCamera::new_unless_unsupported(Supported::Unsupported)
            .unwrap_err()
    );
}

#[test]
fn unsupported_but_no_samples_as_supported_test() {
    assert_eq!(
        "This camera is not supported",
        DecodeableCamera::new_if_supported(Supported::UnsupportedNoSamples)
            .unwrap_err()
    );
}

#[test]
fn unsupported_but_no_samples_as_unless_unsupported_test() {
    assert_eq!(
        "This camera is not supported (explicit)",
        DecodeableCamera::new_unless_unsupported(
            Supported::UnsupportedNoSamples
        )
        .unwrap_err()
    );
}

#[test]
fn unknown_as_supported_test() {
    assert_eq!(
        "This camera is not supported",
        DecodeableCamera::new_if_supported(Supported::Unknown).unwrap_err()
    );
}

#[test]
fn unknown_as_unless_unknown_test() {
    DecodeableCamera::new_unless_unsupported(Supported::Unknown).unwrap();
}

#[test]
fn unknown_but_no_samples_as_supported_test() {
    assert_eq!(
        "This camera is not supported",
        DecodeableCamera::new_if_supported(Supported::UnknownNoSamples)
            .unwrap_err()
    );
}

#[test]
fn unknown_but_no_samples_as_unless_unknown_test() {
    DecodeableCamera::new_unless_unsupported(Supported::UnknownNoSamples)
        .unwrap();
}

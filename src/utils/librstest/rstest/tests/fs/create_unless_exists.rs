use super::super::TestFileSystem;
use super::super::TestLogger;
use crate::vfs::VFS as _;

fn get_args_no_fname() -> Vec<String> {
    vec!["rstest".to_owned(), "-c".to_owned()]
}

fn get_args() -> Vec<String> {
    let mut args = get_args_no_fname();
    args.push("file".to_owned());
    args
}

#[test]
fn no_arguments() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args_no_fname().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert!(fs.log().is_empty());
    assert!(fs.list_files().is_empty());
    assert_eq!(
        vec![
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

#[test]
fn missing_file() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap_err();
    assert!(fs.log().is_empty());
    assert!(fs.list_files().is_empty());
    assert_eq!(
        vec![
            "file                                                   : starting decoding ... ",
            "file failed (0 ms): raw file not found",
            "Total decoding time: 0.000s\n",
            "WARNING: the following 1 tests have failed:",
            "file: raw file not found"
        ],
        log.log()
    );
}

#[test]
fn file_with_no_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    fs.write(fname, &super::super::REF_INPUT).unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert_eq!(vec!["writing 'file.hash'"], fs.log());
    assert_eq!(
        super::super::REF_HASH,
        String::from_utf8(fs.read(fname_hash).unwrap()).unwrap()
    );
    assert_eq!(vec![fname, fname_hash], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : starting decoding ... ",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

#[test]
fn file_with_no_hash_but_with_some_failed_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    let fname_hash_failed = "file.hash.failed";
    fs.write(fname, &super::super::REF_INPUT).unwrap();
    fs.write(fname_hash_failed, "clearly not a failed hash".as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert_eq!(
        vec!["removing 'file.hash.failed'", "writing 'file.hash'"],
        fs.log()
    );
    assert_eq!(
        super::super::REF_HASH,
        String::from_utf8(fs.read(fname_hash).unwrap()).unwrap()
    );
    assert_eq!(vec![fname, fname_hash], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : starting decoding ... ",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

#[test]
fn file_with_wrong_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    fs.write(fname, &super::super::REF_INPUT).unwrap();
    fs.write(fname_hash, "clearly not a hash".as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname, fname_hash], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : hash exists, skipping",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

#[test]
fn file_with_wrong_hash_and_with_some_failed_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    let fname_hash_failed = "file.hash.failed";
    fs.write(fname, &super::super::REF_INPUT).unwrap();
    fs.write(fname_hash, "clearly not a hash".as_bytes())
        .unwrap();
    fs.write(fname_hash_failed, "clearly not a failed hash".as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname, fname_hash, fname_hash_failed], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : hash exists, skipping",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

#[test]
fn file_with_right_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    fs.write(fname, &super::super::REF_INPUT).unwrap();
    fs.write(fname_hash, super::super::REF_HASH.as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname, fname_hash], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : hash exists, skipping",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

#[test]
fn file_with_right_hash_and_with_some_failed_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    let fname_hash_failed = "file.hash.failed";
    fs.write(fname, &super::super::REF_INPUT).unwrap();
    fs.write(fname_hash, super::super::REF_HASH.as_bytes())
        .unwrap();
    fs.write(fname_hash_failed, "clearly not a failed hash".as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname, fname_hash, fname_hash_failed], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : hash exists, skipping",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

// --

#[test]
fn bad_file_with_no_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    fs.write(fname, "".as_bytes()).unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap_err();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : starting decoding ... ",
            "file failed (0 ms): failure when computing hash: RawParserError(DecoderError(Input buffer must be non-empty))",
            "Total decoding time: 0.000s\n",
            "WARNING: the following 1 tests have failed:",
            "file: failure when computing hash: RawParserError(DecoderError(Input buffer must be non-empty))"
        ],
        log.log()
    );
}

#[test]
fn bad_file_with_no_hash_but_with_some_failed_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash_failed = "file.hash.failed";
    fs.write(fname, "".as_bytes()).unwrap();
    fs.write(fname_hash_failed, "clearly not a failed hash".as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap_err();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname, fname_hash_failed], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : starting decoding ... ",
            "file failed (0 ms): failure when computing hash: RawParserError(DecoderError(Input buffer must be non-empty))",
            "Total decoding time: 0.000s\n",
            "WARNING: the following 1 tests have failed:",
            "file: failure when computing hash: RawParserError(DecoderError(Input buffer must be non-empty))"
        ],
        log.log()
    );
}

#[test]
fn bad_file_with_wrong_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    fs.write(fname, "".as_bytes()).unwrap();
    fs.write(fname_hash, "clearly not a hash".as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname, fname_hash], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : hash exists, skipping",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

#[test]
fn bad_file_with_wrong_hash_and_with_some_failed_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    let fname_hash_failed = "file.hash.failed";
    fs.write(fname, "".as_bytes()).unwrap();
    fs.write(fname_hash, "clearly not a hash".as_bytes())
        .unwrap();
    fs.write(fname_hash_failed, "clearly not a failed hash".as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname, fname_hash, fname_hash_failed], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : hash exists, skipping",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

#[test]
fn bad_file_with_right_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    fs.write(fname, "".as_bytes()).unwrap();
    fs.write(fname_hash, super::super::REF_HASH.as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname, fname_hash], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : hash exists, skipping",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

#[test]
fn bad_file_with_right_hash_and_with_some_failed_hash() {
    let mut log = TestLogger::new();
    let mut fs = TestFileSystem::new();
    let fname = "file";
    let fname_hash = "file.hash";
    let fname_hash_failed = "file.hash.failed";
    fs.write(fname, "".as_bytes()).unwrap();
    fs.write(fname_hash, super::super::REF_HASH.as_bytes())
        .unwrap();
    fs.write(fname_hash_failed, "clearly not a failed hash".as_bytes())
        .unwrap();
    fs.clear_log();
    crate::rstest::main(
        &mut log,
        &mut fs,
        &mut get_args().iter().cloned(),
        super::super::REF_CAMERAS,
    )
    .unwrap();
    assert!(fs.log().is_empty());
    assert_eq!(vec![fname, fname_hash, fname_hash_failed], fs.list_files());
    assert_eq!(
        vec![
            "file                                                   : hash exists, skipping",
            "file                                                   : succeeded (0 ms)",
            "Total decoding time: 0.000s\n",
            "All good, all hashes created!"
        ],
        log.log()
    );
}

use rawspeed_common::bit_transmutation::ToLeBytes;
use rawspeed_demuxers_rawdemuxer::rawdemuxer::RawDemuxer;
use rawspeed_demuxers_rawdemuxer::rawdemuxer::RawDemuxerError;
use rawspeed_memory_nd_slice_procurement::ndsliceprocurement::NDSliceProcurementRequestError;
use rawspeed_metadata_camerametadata::camerametadata::DecodeableCamera;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser;
use rawspeed_metadata_camerasxml_parser::camerasxml_parser::blackareas::BlackArea;
use rawspeed_misc_md5::md5::MD5;
use rawspeed_parsers_rawparser::rawparser::RawParser;
use rawspeed_parsers_rawparser::rawparser::RawParserError;
use rawspeed_std::coord_common::{ColIndex, Coord2D, RowIndex};
use rawspeed_std_ndslice::array2dref::Array2DRef;

use crate::logger::Logger;
use crate::vfs::VFS;

pub trait AsSlice {
    type Element;

    fn as_slice(&self) -> &[Self::Element];
}

impl<T, const N: usize> AsSlice for [T; N] {
    type Element = T;

    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        self.as_slice()
    }
}

#[must_use]
#[inline(never)]
pub fn img_data_hash<T>(img: Array2DRef<'_, T>) -> String
where
    T: Copy + ToLeBytes,
    <T as ToLeBytes>::Output: AsSlice<Element = u8>,
{
    let mut per_row_states = vec![];
    for row in 0..img.num_rows() {
        let row = &img[RowIndex::new(row)];
        let mut hasher = MD5::default();
        for e in row {
            hasher.extend(e.to_le_bytes().as_slice());
        }
        per_row_states.push(hasher.flush());
    }
    let mut hasher = MD5::default();
    for e in per_row_states.iter().flat_map(|e| e.iter()) {
        hasher.extend(e.to_le_bytes().as_slice());
    }
    hasher.flush().into()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hash {
    hash: String,
}

#[expect(clippy::too_many_lines)]
fn img_hash(demux: &dyn RawDemuxer, img: Array2DRef<'_, u16>) -> Hash {
    let hash = format!(
        concat!(
            "make: {make}\n",
            "model: {model}\n",
            "mode: {mode}\n",
            "canonical_make: {canonical_make}\n",
            "canonical_model: {canonical_model}\n",
            "canonical_alias: {canonical_alias}\n",
            "canonical_id: {canonical_id}\n",
            "isoSpeed: {isoSpeed}\n",
            "blackLevel: {blackLevel}\n",
            "whitePoint: {whitePoint}\n",
            "blackLevelSeparate: {blackLevelSeparate}\n",
            "wbCoeffs: {wbCoeffs}\n",
            "colorMatrix: {colorMatrix}\n",
            "isCFA: {isCFA}\n",
            "cfa: {cfa}\n",
            "filters: {filters}\n",
            "bpp: {bpp}\n",
            "cpp: {cpp}\n",
            "dataType: {dataType}\n",
            "dimUncropped: {dimUncropped}\n",
            "dimCropped: {dimCropped}\n",
            "cropOffset: {cropOffset}\n",
            "blackAreas: {blackAreas}\n",
            "fuji_rotation_pos: {fuji_rotation_pos}\n",
            "pixel_aspect_ratio: {pixel_aspect_ratio}\n",
            "badPixelPositions: {badPixelPositions}\n",
            "md5sum of per-line md5sums: {hash}\n",
        ),
        make = demux.make(),
        model = demux.model(),
        mode = demux.mode().unwrap_or(""),
        canonical_make = demux.canonical_make(),
        canonical_model = demux.canonical_model(),
        canonical_alias = demux.canonical_alias(),
        canonical_id = demux.canonical_id(),
        isoSpeed = demux.iso_speed().unwrap_or(0),
        blackLevel = demux
            .blacklevel()
            .map_or("FIXME".to_owned(), |()| unreachable!()),
        whitePoint = demux
            .whitelevel()
            .map_or("FIXME".to_owned(), |()| unreachable!()),
        blackLevelSeparate = demux
            .blacklevel_separate()
            .map_or("FIXME".to_owned(), |()| unreachable!()),
        wbCoeffs = demux
            .wb_coeffs()
            .map_or("FIXME".to_owned(), |()| unreachable!()),
        colorMatrix = demux
            .colormatrix()
            .map_or("FIXME".to_owned(), |()| unreachable!()),
        isCFA = demux
            .is_cfa()
            .map_or("FIXME".to_owned(), |()| unreachable!()),
        cfa = demux.cfa().map_or("FIXME".to_owned(), |()| unreachable!()),
        filters = demux
            .filters()
            .map_or("FIXME".to_owned(), |()| unreachable!()),
        bpp = demux.bpp().map_or("FIXME".to_owned(), |()| unreachable!()),
        cpp = demux.cpp(),
        dataType = demux.datatype() as u8,
        dimUncropped = {
            let dim = demux.dim_uncropped();
            format!("{}x{}", *dim.row_len(), *dim.row_count())
        },
        dimCropped = demux
            .dim_cropped()
            .map_or("FIXME".to_owned(), |()| unreachable!()),
        cropOffset = {
            let pos = demux
                .crop_offset()
                .unwrap_or(Coord2D::new(RowIndex::new(0), ColIndex::new(0)));
            format!("{}x{}", *pos.col(), *pos.row())
        },
        blackAreas = {
            let mut repr = String::new();
            if let Some(a) = demux.black_areas() {
                for e in a {
                    use core::fmt::Write as _;
                    let (is_vertical, b, c) = match e {
                        BlackArea::Vertical(vertical) => {
                            (1, **vertical.x, **vertical.width)
                        }
                        BlackArea::Horizontal(horizontal) => {
                            (0, **horizontal.y, **horizontal.height)
                        }
                        _ => unreachable!(),
                    };
                    write!(repr, "{is_vertical}:{b}x{c}, ").unwrap();
                }
            }
            repr
        },
        fuji_rotation_pos = demux.fuji_rotation_pos().unwrap_or(0),
        pixel_aspect_ratio =
            format!("{:.6}", demux.pixel_aspect_ratio().unwrap_or(1.)),
        badPixelPositions = {
            assert!(demux.bad_pixel_positions().is_empty());
            ""
        },
        hash = img_data_hash(img)
    );
    Hash { hash }
}

#[derive(Debug, Clone, Copy)]
enum HashfileHandlingMode {
    VerifyIfExists,
    DecodeAndVerifyIfExists,
    CreateUnlessExists,
    Recreate,
}

#[derive(Debug, Clone, Copy)]
enum Hashfile {
    Exists,
    DoesNotExist,
}

#[derive(Debug)]
#[expect(clippy::enum_variant_names)]
enum HashError {
    IoError(std::io::Error),
    RawParserError(RawParserError),
    RawDemuxerError(RawDemuxerError),
    NDSliceProcurementRequestError(NDSliceProcurementRequestError),
}

impl From<std::io::Error> for HashError {
    fn from(value: std::io::Error) -> Self {
        HashError::IoError(value)
    }
}

impl From<RawParserError> for HashError {
    fn from(value: RawParserError) -> Self {
        HashError::RawParserError(value)
    }
}

impl From<RawDemuxerError> for HashError {
    fn from(value: RawDemuxerError) -> Self {
        HashError::RawDemuxerError(value)
    }
}

impl From<NDSliceProcurementRequestError> for HashError {
    fn from(value: NDSliceProcurementRequestError) -> Self {
        HashError::NDSliceProcurementRequestError(value)
    }
}

#[expect(clippy::missing_trait_methods)]
impl core::error::Error for HashError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            HashError::IoError(error) => error.source(),
            HashError::RawParserError(_)
            | HashError::RawDemuxerError(_)
            | HashError::NDSliceProcurementRequestError(_) => None,
        }
    }
}

impl core::fmt::Display for HashError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HashError::IoError(error) => write!(f, "{error}"),
            HashError::RawParserError(error) => {
                write!(f, "{error}")
            }
            HashError::RawDemuxerError(error) => {
                write!(f, "{error}")
            }
            HashError::NDSliceProcurementRequestError(error) => {
                write!(f, "{error}")
            }
        }
    }
}

#[derive(Debug)]
enum ProcessError {
    FmtError(core::fmt::Error),
    IoError(std::io::Error),
    Utf8Error(std::string::FromUtf8Error),
    RawFileNotFound,
    HashComputationFailure(HashError),
    HashMismatch,
}

impl From<core::fmt::Error> for ProcessError {
    fn from(value: core::fmt::Error) -> Self {
        ProcessError::FmtError(value)
    }
}

impl From<std::io::Error> for ProcessError {
    fn from(value: std::io::Error) -> Self {
        ProcessError::IoError(value)
    }
}

impl From<std::string::FromUtf8Error> for ProcessError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        ProcessError::Utf8Error(value)
    }
}

impl From<HashError> for ProcessError {
    fn from(value: HashError) -> Self {
        ProcessError::HashComputationFailure(value)
    }
}

#[expect(clippy::missing_trait_methods)]
impl core::error::Error for ProcessError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            ProcessError::FmtError(process_error) => process_error.source(),
            ProcessError::IoError(process_error) => process_error.source(),
            ProcessError::Utf8Error(process_error) => process_error.source(),
            ProcessError::HashComputationFailure(process_error) => {
                process_error.source()
            }
            ProcessError::RawFileNotFound | ProcessError::HashMismatch => None,
        }
    }
}

impl core::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ProcessError::FmtError(error) => {
                write!(f, "failure when computing hash: {error}")
            }
            ProcessError::IoError(error) => {
                write!(f, "failure when computing hash: {error}")
            }
            ProcessError::Utf8Error(error) => {
                write!(f, "failure when computing hash: {error}")
            }
            ProcessError::RawFileNotFound => write!(f, "raw file not found"),
            ProcessError::HashComputationFailure(error) => {
                write!(f, "failure when computing hash: {error}")
            }
            ProcessError::HashMismatch => write!(f, "hash/metadata mismatch"),
        }
    }
}

fn compute_hash_for_file_impl(
    fs: &mut dyn VFS,
    file_path: &str,
    cameras: &camerasxml_parser::Cameras<'_>,
) -> Result<Hash, HashError> {
    let input = fs.read(file_path)?;
    let (res, out_buf_request) = RawParser::get_decoder(
        &input,
        cameras,
        DecodeableCamera::new_unless_unsupported,
    )?;
    let mut output_buf = out_buf_request.fulfill()?;
    let mut output = output_buf.get_mut();
    res.decode(&mut output)?;
    let output: Array2DRef<'_, u16> = output.into();
    Ok(img_hash(&*res, output))
}

fn compute_hash_for_file(
    fs: &mut dyn VFS,
    file_path: &str,
    cameras: &camerasxml_parser::Cameras<'_>,
) -> Result<Hash, ProcessError> {
    Ok(match compute_hash_for_file_impl(fs, file_path, cameras) {
        Ok(hash) => hash,
        Err(err) => {
            use core::fmt::Write as _;
            let mut hash = String::new();
            writeln!(hash, "{err}")?;
            return Err(ProcessError::HashComputationFailure(err));
        }
    })
}

const FILENAME_WIDTH: usize = 55;

fn process_file(
    log: &mut dyn Logger,
    fs: &mut dyn VFS,
    file_path: &str,
    opts: HashfileHandlingMode,
    cameras: &camerasxml_parser::Cameras<'_>,
) -> Result<(), ProcessError> {
    let hashfile_name = format!("{file_path}.hash");
    let hashfile_failed_name = format!("{hashfile_name}.failed");
    let hashfile_exists = if fs.exists(&hashfile_name)? {
        Hashfile::Exists
    } else {
        Hashfile::DoesNotExist
    };

    match (opts, hashfile_exists) {
        (HashfileHandlingMode::CreateUnlessExists, Hashfile::Exists) => {
            log.write(
                format!("{file_path:FILENAME_WIDTH$}: hash exists, skipping")
                    .as_str(),
            );
            return Ok(());
        }
        (HashfileHandlingMode::VerifyIfExists, Hashfile::DoesNotExist) => {
            log.write(
                format!("{file_path:FILENAME_WIDTH$}: hash missing, skipping")
                    .as_str(),
            );
            return Ok(());
        }
        (HashfileHandlingMode::VerifyIfExists, Hashfile::Exists)
        | (HashfileHandlingMode::CreateUnlessExists, Hashfile::DoesNotExist)
        | (
            HashfileHandlingMode::DecodeAndVerifyIfExists
            | HashfileHandlingMode::Recreate,
            _,
        ) => {}
    }

    log.write(
        format!("{file_path:FILENAME_WIDTH$}: starting decoding ... ").as_str(),
    );

    if !fs.exists(file_path)? {
        return Err(ProcessError::RawFileNotFound);
    }

    let curr_hash = compute_hash_for_file(fs, file_path, cameras)?;
    if fs.exists(&hashfile_failed_name)? {
        fs.remove_file(&hashfile_failed_name)?;
    }

    match (opts, hashfile_exists) {
        (
            HashfileHandlingMode::DecodeAndVerifyIfExists,
            Hashfile::DoesNotExist,
        ) => {}

        (HashfileHandlingMode::CreateUnlessExists, Hashfile::DoesNotExist)
        | (HashfileHandlingMode::Recreate, _) => {
            fs.write(&hashfile_name, curr_hash.hash.as_bytes())?;
        }

        (
            HashfileHandlingMode::VerifyIfExists
            | HashfileHandlingMode::DecodeAndVerifyIfExists,
            Hashfile::Exists,
        ) => {
            let prev_hash = Hash {
                hash: String::from_utf8(fs.read(&hashfile_name)?)?,
            };
            if curr_hash != prev_hash {
                fs.write(&hashfile_failed_name, curr_hash.hash.as_bytes())?;
                return Err(ProcessError::HashMismatch);
            }
        }

        (HashfileHandlingMode::VerifyIfExists, Hashfile::DoesNotExist)
        | (HashfileHandlingMode::CreateUnlessExists, Hashfile::Exists) => {
            unreachable!()
        }
    }

    Ok(())
}

struct FailedTest<'a> {
    file: &'a str,
    err: ProcessError,
}

impl<'a> FailedTest<'a> {
    const fn new(file: &'a str, err: ProcessError) -> Self {
        Self { file, err }
    }
}

#[derive(Debug)]
struct AggregatedError;

#[expect(clippy::missing_trait_methods)]
impl core::error::Error for AggregatedError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
}

impl core::fmt::Display for AggregatedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "You have failed tests!")
    }
}

fn results(
    log: &mut dyn Logger,
    failed_tests: &[FailedTest<'_>],
    opts: HashfileHandlingMode,
) -> Result<(), AggregatedError> {
    if failed_tests.is_empty() {
        let mut msg = ("All good, ").to_owned();
        match opts {
            HashfileHandlingMode::VerifyIfExists
            | HashfileHandlingMode::DecodeAndVerifyIfExists => {
                msg.push_str("no tests failed!");
            }
            HashfileHandlingMode::CreateUnlessExists
            | HashfileHandlingMode::Recreate => {
                msg.push_str("all hashes created!");
            }
        }
        log.write(msg.as_str());
        return Ok(());
    }

    log.write(
        format!(
            "WARNING: the following {} tests have failed:",
            failed_tests.len()
        )
        .as_str(),
    );
    for failed_test in failed_tests {
        log.write(
            format!("{}: {}", failed_test.file, failed_test.err).as_str(),
        );
    }
    Err(AggregatedError {})
}

fn process(
    log: &mut dyn Logger,
    fs: &mut dyn VFS,
    files: &[String],
    opts: HashfileHandlingMode,
    cameras: &camerasxml_parser::Cameras<'_>,
) -> Result<(), AggregatedError> {
    let mut elapsed_total = core::time::Duration::default();
    let mut failed_tests = vec![];
    for file_path in files {
        let now = std::time::Instant::now();
        let res = process_file(log, fs, file_path, opts, cameras);
        let elapsed = now.elapsed();
        elapsed_total += elapsed;
        if let Err(err) = res {
            log.write(
                format!(
                    "{file_path} failed ({} ms): {err}",
                    elapsed.as_millis()
                )
                .as_str(),
            );
            failed_tests.push(FailedTest::new(file_path, err));
        } else {
            log.write(
                format!(
                    "{file_path:FILENAME_WIDTH$}: succeeded ({} ms)",
                    elapsed.as_millis()
                )
                .as_str(),
            );
        }
    }

    log.write(
        format!("Total decoding time: {:.3}s\n", elapsed_total.as_secs_f64())
            .as_str(),
    );
    results(log, &failed_tests, opts)
}

#[must_use]
#[inline]
pub const fn get_camerasxml_path() -> &'static str {
    let camerasxml_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../rawspeed/data/cameras.xml"
    );
    camerasxml_path
}

#[inline(never)]
pub fn main(
    log: &mut dyn Logger,
    fs: &mut dyn VFS,
    args: &mut dyn Iterator<Item = String>,
    camerasxml_contents: &str,
) -> Result<(), Box<dyn core::error::Error>> {
    let cameras = camerasxml_parser::parse_str(camerasxml_contents)?;

    let mut opt_create = false;
    let mut opt_force = false;
    let args: Vec<String> = args
        .filter(|arg: &String| {
            let arg: &str = arg;
            match arg {
                "-c" => {
                    opt_create = true;
                }
                "-f" => {
                    opt_force = true;
                }
                _ => return true,
            }
            false
        })
        .collect();

    let opts = match (opt_create, opt_force) {
        (false, false) => HashfileHandlingMode::VerifyIfExists,
        (false, true) => HashfileHandlingMode::DecodeAndVerifyIfExists,
        (true, false) => HashfileHandlingMode::CreateUnlessExists,
        (true, true) => HashfileHandlingMode::Recreate,
    };

    let files = args.get(1..).ok_or("No arguments provided")?;

    process(log, fs, files, opts, &cameras)?;

    Ok(())
}

#[cfg(test)]
mod tests;

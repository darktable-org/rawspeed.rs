#[cfg(test)]
mod tests {
    #[test]
    #[cfg_attr(miri, ignore)]
    fn xmllint() -> Result<(), Box<dyn core::error::Error>> {
        use rawspeed_metadata_camerasxml_parser::camerasxml_parser;
        let camerasxml_path =
            rawspeed_utils_librstest::rstest::get_camerasxml_path();

        let camerasxml_contents = std::fs::read_to_string(camerasxml_path)?;

        let _repr = camerasxml_parser::parse_str(&camerasxml_contents)?;

        Ok(())
    }
}

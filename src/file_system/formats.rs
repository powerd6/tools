use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub(crate) enum Format {
    Json,
    Yaml,
    Plaintext,
}

impl From<PathBuf> for Format {
    fn from(value: PathBuf) -> Self {
        match value.extension().and_then(|e| e.to_str()) {
            Some("json") => Format::Json,
            Some("yaml") | Some("yml") => Format::Yaml,
            Some("md") | Some("txt") => Format::Plaintext,
            _ => panic!("Unsupported file format"),
        }
    }
}

trait FormatData {
    fn get_data() {
        todo!("Determine return value and implement variants");
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Format;

    use rstest::rstest;

    use crate::file_system::FileSystem;

    use crate::RealFileSystem;
    use testdir::testdir;

    #[rstest]
    #[case("json", Format::Json)]
    #[case("yaml", Format::Yaml)]
    #[case("yml", Format::Yaml)]
    #[case("md", Format::Plaintext)]
    #[case("txt", Format::Plaintext)]
    fn it_choses_the_correct_file_format(#[case] extension: String, #[case] expected: Format) {
        let fs = RealFileSystem;

        let dir: PathBuf = testdir!();

        let file = fs
            .create_file(&dir.join(format!("file.{}", extension)), "")
            .unwrap();
        assert_eq!(Format::from(file), expected)
    }

    #[test]
    #[should_panic]
    fn it_panics_when_invalid() {
        let fs = RealFileSystem;
        let dir: PathBuf = testdir!();

        let file = fs.create_file(&dir.join("file._"), "").unwrap();
        let _will_panic = Format::from(file);
    }
}

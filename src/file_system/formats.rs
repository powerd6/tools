use std::path::PathBuf;

use phf::{phf_ordered_map, OrderedMap};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Format {
    Json,
    Yaml,
    Plaintext,
}

impl From<PathBuf> for Format {
    fn from(value: PathBuf) -> Self {
        value
            .extension()
            .and_then(|e| e.to_str())
            .and_then(|ext| EXTENSIONS.get(ext))
            .unwrap()
            .clone()
    }
}

pub(crate) static EXTENSIONS: OrderedMap<&str, Format> = phf_ordered_map! {
    "json" => Format::Json,
    "yaml" => Format::Yaml,
    "yml" => Format::Yaml,
    "txt" => Format::Plaintext,
    "md" => Format::Plaintext,
};

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

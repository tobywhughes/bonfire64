#[derive(Debug)]
pub struct FileMetadata {
  pub filename: String,
  pub base: String,
  pub extension: String,
}

impl FileMetadata {
  pub fn new(filename: &String) -> FileMetadata {
    let (base, extension) = split_filename(&filename);

    FileMetadata {
      filename: filename.to_string(),
      base: base.to_string(),
      extension: extension.to_string(),
    }
  }
}

fn split_filename(name: &String) -> (&str, &str) {
  let split: Vec<&str> = name.rsplitn(2, ".").collect();

  return (split[1], split[0]);
}

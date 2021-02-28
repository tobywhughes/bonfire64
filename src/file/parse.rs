pub fn parse_extension(filename: &String) -> &str {
  let split: Vec<&str> = filename.rsplitn(2, '.').collect();
  split[0]
}

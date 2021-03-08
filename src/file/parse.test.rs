use super::*;

#[test]
fn test_parse_extension_normal() {
  let test_string = String::from("file.z64");
  let extension_expected = "z64";

  let extension_result = parse_extension(&test_string);

  assert_eq!(extension_result, extension_expected);
}

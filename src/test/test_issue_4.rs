use crate::*;
use tempfile::TempDir;

#[test]
fn issue_4_mp3_encode_and_decode() {
  // Tags to set
  const TAG_TITLE_STR: &str = "some_title";
  const TAG_COPYRIGHT: &str = "dobby_is_free";
  const TAG_COMMENT: &str = "no comment";

  // Empty data vec
  const DEFAULT_BUF: [i16; 256] = [0i16; 256];

  let tmp_dir = TempDir::new().unwrap();
  let tmp_path = tmp_dir.as_ref().join("issue_4_encode_and_decode.mp3");

  // Write the file
  {
    let mut snd = OpenOptions::WriteOnly(WriteOptions::new(
      MajorFormat::MPEG,
      SubtypeFormat::MPEG_LAYER_III,
      Endian::File,
      8000,
      2,
    ))
    .from_path(&tmp_path)
    .unwrap();
    for _ in 0..256 {
      snd.write_from_slice(&DEFAULT_BUF).unwrap();
    }
    snd.set_tag(TagType::Title, TAG_TITLE_STR).unwrap();
    snd.set_tag(TagType::Copyright, TAG_COPYRIGHT).unwrap();
    snd.set_tag(TagType::Comment, TAG_COMMENT).unwrap();
  }

  // Check the file
  {
    let snd = OpenOptions::ReadOnly(ReadOptions::Auto)
      .from_path(&tmp_path)
      .unwrap();
    
    // Check the tags has been set
    assert_eq!(snd.get_tag(TagType::Title).unwrap(), TAG_TITLE_STR);
    assert_eq!(snd.get_tag(TagType::Copyright).unwrap(), TAG_COPYRIGHT);
    assert_eq!(snd.get_tag(TagType::Comment).unwrap(), TAG_COMMENT);

    // Check the missing tags returns None
    assert_eq!(snd.get_tag(TagType::Software), None);
    assert_eq!(snd.get_tag(TagType::Artist), None);
    assert_eq!(snd.get_tag(TagType::Date), None);
    assert_eq!(snd.get_tag(TagType::Album), None);
    assert_eq!(snd.get_tag(TagType::License), None);
    assert_eq!(snd.get_tag(TagType::Tracknumber), None);
    assert_eq!(snd.get_tag(TagType::Genre), None);
  }
  std::fs::remove_file(&tmp_path).unwrap();
}
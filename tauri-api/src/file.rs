mod extract;
mod file_move;

use std::fs;

use crate::Error;

pub use extract::*;
pub use file_move::*;

pub fn read_string(file: String) -> crate::Result<String> {
  fs::read_to_string(file)
    .map_err(|err| Error::File(format!("Read_string failed: {}", err)).into())
    .map(|c| c)
}

pub fn read_binary(file: String) -> crate::Result<Vec<u8>> {
  fs::read(file)
    .map_err(|err| Error::File(format!("Read_binary failed: {}", err)).into())
    .map(|b| b)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::Error;
  use totems::{assert_err, assert_ok};

  #[test]
  fn check_read_string() {
    let file = String::from("test/test.txt");

    let res = read_string(file);

    assert_ok!(res);

    if let Ok(s) = res {
      assert_eq!(s, "This is a test doc!".to_string());
    }
  }

  #[test]
  fn check_read_string_fail() {
    let file = String::from("test/");

    let res = read_string(file);

    assert_err!(res);

    if let Some(Error::File(e)) = res.unwrap_err().downcast_ref::<Error>() {
      #[cfg(windows)]
      assert_eq!(
        *e,
        "Read_string failed: Access is denied. (os error 5)".to_string()
      );
      #[cfg(not(windows))]
      assert_eq!(
        *e,
        "Read_string failed: Is a directory (os error 21)".to_string()
      );
    }
  }

  #[test]
  fn check_read_binary() {
    let file = String::from("test/test_binary");

    #[cfg(windows)]
    let expected_vec = vec![
      35, 33, 47, 98, 105, 110, 47, 98, 97, 115, 104, 13, 10, 13, 10, 101, 99, 104, 111, 32, 34,
      72, 101, 108, 108, 111, 32, 116, 104, 101, 114, 101, 34,
    ];
    #[cfg(not(windows))]
    let expected_vec = vec![
      35, 33, 47, 98, 105, 110, 47, 98, 97, 115, 104, 10, 10, 101, 99, 104, 111, 32, 34, 72, 101,
      108, 108, 111, 32, 116, 104, 101, 114, 101, 34,
    ];

    let res = read_binary(file);

    assert_ok!(res);

    if let Ok(vec) = res {
      assert_eq!(vec, expected_vec);
    }
  }

  #[test]
  fn check_read_binary_fail() {
    let file = String::from("test/");

    let res = read_binary(file);

    assert_err!(res);

    if let Some(Error::File(e)) = res.unwrap_err().downcast_ref::<Error>() {
      #[cfg(windows)]
      assert_eq!(
        *e,
        "Read_binary failed: Access is denied. (os error 5)".to_string()
      );
      #[cfg(not(windows))]
      assert_eq!(
        *e,
        "Read_binary failed: Is a directory (os error 21)".to_string()
      );
    }
  }
}

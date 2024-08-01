use std::error;
use std::io::{BufRead, Error, ErrorKind};

use crate::format::Todo;

pub(crate) fn done<R: BufRead>(
    reader: &mut R,
    i: u32,
    t: f32,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut w = String::new();

    let mut index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        if index == i {
            let mut todo = Todo::deserialize(l.as_str())?;
            if !todo.done {
                todo.time = Some(t + todo.time.unwrap_or(0.0));
                if todo.time < Some(0.0) {
                    todo.time = Some(todo.time.unwrap_or(0.0) + 24.0);
                }
            }
            todo.done = true;
            // println!("{}", Some(t).unwrap_or(0.0));
            w.push_str(todo.serialize().as_str());
        } else {
            w.push_str(l.as_str());
        }

        index += 1;
        l.clear();
    }

    if index <= i {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "invalid index",
        )));
    }

    Ok(w)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::io::BufReader;
//
//     #[test]
//     fn test_done() {
//         let mut reader = BufReader::new(
//             "[ ] first ()\n\
//              [ ] second ()\n"
//                 .as_bytes(),
//         );
//         assert_eq!(
//             done(&mut reader, 0).unwrap(),
//             "[x] first ()\n\
//              [ ] second ()\n"
//         );
//     }
// }

use std::error;
use std::io::BufRead;

use crate::cli::Part;
use crate::format::Todo;
use crate::string::StringExt;

const TODO: &str = "\u{2610}";
const DONE: &str = "\u{2611}";

struct List(Todo);
impl List {
    fn serialize(&self, index: u32) -> String {
        if self.0.done && self.0.time.is_some() {
            format!(
                "{} {:03}: {} ({:.1})\n",
                DONE,
                index,
                self.0.task,
                self.0.time.unwrap_or(0.0)
            )
            .cyan()
        } else if self.0.done {
            format!("{} {:03}: {}\n", DONE, index, self.0.task).cyan()
        } else if !self.0.done && self.0.time.is_some() {
            format!(
                "{} {:03}: {} ({:.1})\n",
                TODO,
                index,
                self.0.task,
                self.0.time.unwrap_or(0.0)
            )
        } else {
            format!("{} {:03}: {}\n", TODO, index, self.0.task)
        }
        .bold()
    }
}

pub(crate) fn list<R: BufRead>(
    reader: &mut R,
    s: &Part,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    // let mut w = String::new();
    // let mut todo_sort = String::new();
    // let mut done = String::new();
    // let mut time = String::new();

    let mut index = 0;
    let mut l = String::new();

    let mut done_w = String::new();
    let mut time_w = String::new();
    let mut todo_w = String::new();

    while reader.read_line(&mut l)? > 0 {
        // println!("l :{}", l);
        let task = Todo::deserialize(l.as_str())?;
        if task.done {
            // done.push_str(l.as_str());
            let done_l = List(Todo::deserialize(l.as_str())?);
            done_w.push_str(done_l.serialize(index).as_str());
        } else if task.time.is_some() {
            // time.push_str(l.as_str());
            let time_l = List(Todo::deserialize(l.as_str())?);
            time_w.push_str(time_l.serialize(index).as_str());
        } else {
            // todo_sort.push_str(l.as_str());
            let todo_l = List(Todo::deserialize(l.as_str())?);
            todo_w.push_str(todo_l.serialize(index).as_str());
        }
        // println!("todo_w :{}", todo_w);

        // let todo = List(Todo::deserialize(l.as_str())?);
        // println!("todo :{}", todo.serialize(index).as_str());
        //
        // if !todo_sort.as_str().is_empty() {
        //     let todo_l = List(Todo::deserialize(todo_sort.as_str())?);
        //     todo_w.push_str(todo_l.serialize(index).as_str());
        // }
        // if !done.as_str().is_empty() {
        //     let done_l = List(Todo::deserialize(done.as_str())?);
        //     done_w.push_str(done_l.serialize(index).as_str());
        // }
        // if !time.as_str().is_empty() {
        //     let time_l = List(Todo::deserialize(time.as_str())?);
        //     time_w.push_str(time_l.serialize(index).as_str());
        // }
        index += 1;
        l.clear();
        // println!("1{}", todo_w);
    }

    // Ok(w)
    match *s {
        Part::All => Ok(time_w + todo_w.as_str() + &done_w),
        Part::Todo => Ok(todo_w.as_str().to_owned() + &done_w),
        Part::Done => Ok(time_w + todo_w.as_str()),
        Part::Doing => Ok(time_w),
        // Some(_) => Ok(time_w + todo_w.as_str() + &done_w),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_list_all() {
        let mut reader = BufReader::new(
            "[x] first ()\n\
             [x] second (2.0)\n\
             [ ] third ()\n\
             [ ] fourth (4.0)\n"
                .as_bytes(),
        );
        assert_eq!(
            list(&mut reader, &Part::All).unwrap(),
            "\u{1b}[1m\u{2610} 003: fourth (4.0)\n\u{1b}[0m\
             \u{1b}[1m\u{2610} 002: third\n\u{1b}[0m\
             \u{1b}[1m\u{1b}[36m\u{2611} 000: first\n\u{1b}[0m\u{1b}[0m\
             \u{1b}[1m\u{1b}[36m\u{2611} 001: second (2.0)\n\u{1b}[0m\u{1b}[0m"
        );
    }
    #[test]
    fn test_list_todo() {
        let mut reader = BufReader::new(
            "[x] first ()\n\
             [x] second (2.0)\n\
             [ ] third ()\n\
             [ ] fourth (4.0)\n"
                .as_bytes(),
        );
        assert_eq!(
            list(&mut reader, &Part::Todo).unwrap(),
            "\u{1b}[1m\u{2610} 002: third\n\u{1b}[0m\
             \u{1b}[1m\u{1b}[36m\u{2611} 000: first\n\u{1b}[0m\u{1b}[0m\
             \u{1b}[1m\u{1b}[36m\u{2611} 001: second (2.0)\n\u{1b}[0m\u{1b}[0m"
        );
    }
    #[test]
    fn test_list_done() {
        let mut reader = BufReader::new(
            "[x] first ()\n\
             [x] second (2.0)\n\
             [ ] third ()\n\
             [ ] fourth (4.0)\n"
                .as_bytes(),
        );
        assert_eq!(
            list(&mut reader, &Part::Done).unwrap(),
            "\u{1b}[1m\u{2610} 003: fourth (4.0)\n\u{1b}[0m\
            \u{1b}[1m\u{2610} 002: third\n\u{1b}[0m"
        );
    }
    #[test]
    fn test_list_doing() {
        let mut reader = BufReader::new(
            "[x] first ()\n\
             [x] second (2.0)\n\
             [ ] third ()\n\
             [ ] fourth (4.0)\n"
                .as_bytes(),
        );
        assert_eq!(
            list(&mut reader, &Part::Doing).unwrap(),
            "\u{1b}[1m\u{2610} 003: fourth (4.0)\n\u{1b}[0m"
        );
    }
}

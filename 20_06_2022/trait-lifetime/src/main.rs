// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime

// Use String instead of &str

// use std::fmt;
// struct StrDisplayable(Vec<String>);

// impl fmt::Display for StrDisplayable {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for v in &self.0 {
//             write!(f, "\n{}", v)?;
//         }
//         Ok(())
//     }
// }

// fn main() {
//     let vec: Vec<String> = vec!["a".to_string(), "bc".to_string(), "def".to_string()];
//     let vec_foo = StrDisplayable(vec);
//     println!("{}", vec_foo);
// }

//==============================================================================================

// Add Lifetime anotation for struct

use std::fmt;
struct StrDisplayable<'a>(Vec<&'a str>);

impl<'a> fmt::Display for StrDisplayable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

fn main() {
    let vec: Vec<&str> = vec!["a", "bc", "def"];
    let vec_foo = StrDisplayable(vec);
    println!("{}", vec_foo);
}

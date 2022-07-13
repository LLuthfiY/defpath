use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{self};
fn main() {
    let cmd_line: Vec<String> = env::args().collect();
    let curdir = String::from(env::current_dir().unwrap().to_str().unwrap());

    let mut temp_exe = env::current_exe().unwrap();
    temp_exe.pop();
    let exe_dir = temp_exe.to_str().unwrap();

    if &cmd_line.len() < &2 {
        println!("How to use this program:");
        println! {"defpath new {{shortcut name}} "}
        println! {"defpath delete {{shortcut name}} "}
        println! {"defpath list"}
        return ();
    };

    if &cmd_line[1] == "new" {
        let mut file_name = "test";
        if &cmd_line.len() > &2 {
            file_name = &cmd_line[2];
        }
        let mut file =
            File::create(path::Path::new(&exe_dir).join(format!("{}.bat", file_name))).unwrap();
        file.write(format!("{}\ncd {}", &curdir[..2], &curdir).as_bytes())
            .unwrap();
        println!("Create {}.bat at {}", file_name, exe_dir);
    }
    if &cmd_line[1] == "delete" {
        let mut file_name = "test";
        if &cmd_line.len() > &2 {
            file_name = &cmd_line[2];
            if file_name.len() > 4 {
                if file_name[file_name.len() - 4..].to_string() == ".bat" {
                    file_name = &file_name[..file_name.len() - 4];
                }
            }
        }
        fs::remove_file(path::Path::new(&exe_dir).join(format!("{}.bat", file_name))).unwrap();
        println!("Delete {}.bat at {}", file_name, exe_dir);
    }
    if &cmd_line[1] == "list" {
        let files = fs::read_dir(path::Path::new(&exe_dir)).unwrap();
        for file in files {
            let file = file.unwrap();
            println!("{}", file.path().to_str().unwrap());
        }
    }
}

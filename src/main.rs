use std::env;
use std::io;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: quick_delete <path>");
        return;
    }
    let path = &args[1];
    if Path::new(path).is_dir() {
        println!("你正在删除一个文件夹，请按回车确认删除...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    let path_vec = vec![path];
    fs_extra::remove_items(&path_vec).unwrap();
}

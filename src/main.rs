use std::env;
use std::io;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage(CMD): fast-delete-rs path");
        return;
    }
    let path = &args[1];
    if Path::new(path).is_dir() {
        println!("Current path: {}", path);
        println!("当前路径: {}", path);
        println!("You are deleting a folder, please press enter to confirm deletion...");
        println!("你正在删除一个文件夹，请按回车确认删除...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    println!("Deleting...");
    println!("删除中...");
    let path_vec = vec![path];
    fs_extra::remove_items(&path_vec).unwrap();
}

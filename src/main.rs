// 从剪贴板中自动复制内容到指定的文件中
// 实现过程
// 创建一个剪贴板读取器
// 循环读取剪贴板内容,如果读取到的内容不为空且和上次读取到的内容不同,则将内容写入文件中

//保存内容到文件中
// 设置路径,将内容附加到文件中

use std::{fs::OpenOptions, io::Write, thread, time::Duration};

use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut clipboard = ClipboardContext::new().expect("msg");
    let mut last_content = String::new();

    loop {
        match clipboard.get_contents() {
            Ok(current_content) => {
                if current_content != last_content && !current_content.is_empty() {
                    save_to_file(&current_content);
                    last_content = current_content;
                }
            }
            Err(e) => {
                eprintln!("Failed to read clipboard: {}", e);
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn save_to_file(content: &str) {
    let path = "/Users/tao/notes/clipboard.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

  file.write_all(content.as_bytes()).expect("Failed to write to file");
}
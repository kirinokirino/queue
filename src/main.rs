use std::fs;
use std::process::Command;
use std::{thread, time};
fn main() {
    let filename = "./queue.txt";
    let update_time = time::Duration::from_millis(1000);

    loop {
        let list = load_and_clear(filename);
        play_list(list);
        thread::sleep(update_time);
    }
}

fn load_and_clear(file: &str) -> Vec<String> {
    let contents =
        fs::read_to_string(file).unwrap_or_else(|_| panic!("Please create a file {}", file));
    let _result = fs::write(file, "");
    let list = contents
        .lines()
        .rev()
        .map(String::from)
        .collect::<Vec<String>>();
    list
}

fn play_list(mut links: Vec<String>) {
    while let Some(link) = links.pop() {
        play_song(link)
    }
}

fn play_song(link: String) {
    let result = Command::new("mpv")
        .arg(link)
        .arg("--ao=jack")
        .arg("--no-video")
        .arg("--jack-port=music")
        .arg("--really-quiet")
        .status();

    match result {
        Ok(_status) => (),
        Err(error) => println!("Can't play requested song! ERROR: {}", error),
    }
}

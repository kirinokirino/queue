#[cfg(target_os = "linux")]
use inotify::{Inotify, WatchMask};
use std::fs;
use std::process::Command;

#[cfg(not(target_os = "linux"))]
use std::{thread, time};

fn main() {
    let filename = "./queue.txt";

    #[cfg(target_os = "linux")]
    {
        let mut inotify = Inotify::init().expect("Error while initializing inotify instance");
        inotify
            .watches()
            .add(filename, WatchMask::MODIFY)
            .expect("Failed to add file watch");

        loop {
            // Read events that were added with `Watches::add` above.
            let mut buffer = [0; 1024];
            let events = inotify
                .read_events_blocking(&mut buffer)
                .expect("Error while reading events");

            for event in events {
                let list = load_and_clear(filename);
                play_list(list);
            }
            inotify.read_events_blocking(&mut buffer);
        }
    }

    #[cfg(not(target_os = "linux"))]
    loop {
        let list = load_and_clear(filename);
        play_list(list);
        thread::sleep(time::Duration::from_millis(1000));
        //println!("queue is working?...");
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
        //.arg("--ao=alsa")
        .arg("--no-video")
        .arg("--volume=70")
        //.arg("--jack-port=music")
        //.arg("--msg-module")
        .arg("--msg-level=all=warn,file=v")
        .status();

    match result {
        Ok(_status) => (),
        Err(error) => println!("Can't play requested song! ERROR: {}", error),
    }
}

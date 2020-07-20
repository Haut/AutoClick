extern crate winapi;
use winapi::um::winuser::{MOUSEEVENTF_LEFTDOWN, INPUT, INPUT_MOUSE, SendInput, MOUSEEVENTF_LEFTUP, MOUSEINPUT, INPUT_u};
use clap::{App, Arg};
use std::time::Duration;
use std::thread::sleep;

fn args() -> App<'static, 'static> {
    App::new("AutoClick")
        .version("0.1.0")
        .arg(Arg::with_name("follow_type")
            .long("follow_type")
            .short("f")
            .takes_value(true)
            .value_name("FOLLOW_TYPE")
            .default_value("follow")
            .help("Determines whether the clicks follow the mouse or not")
            .possible_values(&["static", "follow"]))
        .arg(Arg::with_name("milliseconds")
            .long("milliseconds")
            .short("m")
            .takes_value(true)
            .value_name("MILLISECONDS")
            .default_value("3000")
            .help("Length of time between clicks in milliseconds"))
}

fn main() {
    let matches = args().get_matches();
    let _click_type = matches.value_of("follow_type").unwrap().eq("follow");
    let ms = match matches.value_of("milliseconds").unwrap().parse::<u64>() {
        Ok(ms) => ms,
        Err(_) => 3000 as u64
    };
    let wait_duration = Duration::from_millis(ms);
    loop {
        sleep(wait_duration);
        unsafe { left_click(); }
    }
}

const LEFT_DOWN: MOUSEINPUT = MOUSEINPUT {
    dx: 0,
    dy: 0,
    mouseData: 0,
    dwFlags: MOUSEEVENTF_LEFTDOWN,
    time: 0,
    dwExtraInfo: 0
};

const LEFT_UP: MOUSEINPUT = MOUSEINPUT {
    dx: 0,
    dy: 0,
    mouseData: 0,
    dwFlags: MOUSEEVENTF_LEFTUP,
    time: 0,
    dwExtraInfo: 0
};

unsafe fn left_click() {
    let mut input_u: INPUT_u = std::mem::zeroed();
    *input_u.mi_mut() = LEFT_DOWN;
    let mut input = INPUT { type_: INPUT_MOUSE, u: input_u };
    let size = std::mem::size_of::<INPUT>() as i32;
    SendInput(1, &mut input, size);

    let mut input_u: INPUT_u = std::mem::zeroed();
    *input_u.mi_mut() = LEFT_UP;
    let mut input = INPUT { type_: INPUT_MOUSE, u: input_u };
    let size = std::mem::size_of::<INPUT>() as i32;
    SendInput(1, &mut input, size);
}

extern crate termion;
extern crate time;


const FRAME: &'static str = include_str!("../assets/frame.txt");

const DIGITS: [&'static str; 10] = [
    include_str!("../assets/digit-0.txt"),
    include_str!("../assets/digit-1.txt"),
    include_str!("../assets/digit-2.txt"),
    include_str!("../assets/digit-3.txt"),
    include_str!("../assets/digit-4.txt"),
    include_str!("../assets/digit-5.txt"),
    include_str!("../assets/digit-6.txt"),
    include_str!("../assets/digit-7.txt"),
    include_str!("../assets/digit-8.txt"),
    include_str!("../assets/digit-9.txt"),
];

fn render_digit (position: (u16, u16), digit: i32) {
    let (x, y) = position;
    for (i, line) in DIGITS[digit as usize].split('\n').enumerate() {
        println!("{}{}", termion::cursor::Goto(x, y + i as u16), line);
    }
}

fn display_time (hour: i32, minute: i32, second: i32) {
    render_digit((4, 4), hour / 10);
    render_digit((8, 4), hour % 10);
    render_digit((14, 4), minute / 10);
    render_digit((18, 4), minute % 10);
    render_digit((24, 4), second / 10);
    render_digit((28, 4), second % 10);
}

fn main() {
    println!("{}", termion::clear::All);
    println!("{}{}", termion::cursor::Goto(1, 1), FRAME);
    loop {
        let now = time::now();
        display_time(now.tm_hour, now.tm_min, now.tm_sec);
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

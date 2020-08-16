extern crate clap;

use clap::{Arg, App, value_t};


pub struct MyCounter{
    pub prefix: String,
    pub start: u32,
    pub end: u32,
    pub step: u32,
    pub current: u32,
}

pub struct Message{
    pub prefix: String,
    pub integer: u32,
    pub percent: f32,
}

// trait练习
pub trait Display{
    fn int_format(&self) -> String;
    fn percent_format(&self) -> String;
}

impl MyCounter {
    fn new(prefix: String, start: u32, end: u32, step: u32) -> MyCounter {
        MyCounter {prefix: prefix, start: start, end: end, step: step, current: start}
    }
}

impl Display for Message {
    fn percent_format(&self) -> String {
        format!("{}: {}", self.prefix, self.percent)
    }

    fn int_format(&self) -> String {
        format!("{}: {}", self.prefix, self.integer)
    }
}

// iter练习
impl Iterator for MyCounter {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let message = Message {prefix: String::from(&self.prefix), integer: self.current, percent: (self.current) as f32 / (self.end - self.start) as f32};
            self.current += self.step;
            Some(message)
        }
        else {
            None
        }
    }
}


// 数数。counter的所有权转移进了这里
fn count(counter: MyCounter, display_percent: bool)  {
    for message in counter {
        if display_percent {
            println!("{}", message.percent_format());
        }
        else {
            println!("{}", message.int_format());
        }
    }
}

fn main() {
    let matches = App::new("app")
      .version("0.0.1")
      .author("Wang Yuanzheng")
      .arg(Arg::with_name("prefix")
            .long("prefix")
            .takes_value(true)
            .help("start number.")
            .required(true)
        )
      .arg(Arg::with_name("start")
            .long("start")
            .takes_value(true)
            .help("start number.")
            .required(true)
        )
      .arg(Arg::with_name("end")
            .long("end")
            .takes_value(true)
            .help("end number.")
            .required(true)
      )
      .arg(Arg::with_name("step")
            .long("step")
            .takes_value(true)
            .help("step.")
            .required(true)
      )
      .arg(Arg::with_name("display_percent")
            .long("display_percent")
            .takes_value(false)
            .help("whether to display in percent format.")
      )
      .get_matches();

    let prefix = String::from(matches.value_of("prefix").unwrap());
    let start = value_t!(matches.value_of("start"), u32).unwrap_or_else(|e| e.exit());
    let end = value_t!(matches.value_of("end"), u32).unwrap_or_else(|e| e.exit());
    let step = value_t!(matches.value_of("step"), u32).unwrap_or_else(|e| e.exit());
    let display_percent  =matches.is_present("display_percent");
    let counter = MyCounter::new(prefix, start, end, step);
    // 所有权借用
    let borrow_counter = &counter;
    println!("prefix: {}, start: {}, end: {}, step: {}", borrow_counter.prefix, borrow_counter.start, borrow_counter.end, borrow_counter.step);
    // 所有权转移进这个函数
    count(counter, display_percent);
    // 由于所有权转移进了函数里，下面再打印这句话，编译会报错"Cannot move out of counter because it is borrowed"
    // println!("prefix: {}, start: {}, end: {}, step: {}", borrow_counter.prefix, borrow_counter.start, borrow_counter.end, borrow_counter.step);
}

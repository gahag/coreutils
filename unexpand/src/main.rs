use std::{
    env::current_dir,
    fs::File,
    io::{prelude::BufRead, stdin, stdout, Stdout, BufReader, Write},
    process,
};

use clap::{load_yaml, App, ArgMatches};

struct Unexpand {
    pub all: bool,
    pub first_only: bool,
    pub tabs: Vec<String>,
    output: Stdout,
}

impl Unexpand {
    pub fn from_matches(matches: &ArgMatches) -> Self {
        let all = matches.is_present("all");
        let first_only = matches.is_present("first_only");

        Unexpand {
            all,
            first_only,
            tabs: matches
                .value_of("tabs")
                .unwrap_or("8")
                .split(",")
                .map(|s| s.to_string())
                .collect(),
            output: stdout()
        }
    }

    pub fn unexpand_line(self: &mut Self, line: String) {
        let mut convert = true;
        let mut spaces: usize = 0;
        let mut column: usize = 0;

        for c in line.bytes() {
            match c {
                b' ' => {
                    spaces += 1;
                }
                b'\x08' => {
                    spaces -= !!spaces;
                    column -= !!column;
                }
                _ => {
                    if spaces > 2 && convert {
                        self.output.write("\t".repeat(spaces / 2).as_bytes()).expect("write error");
                        spaces = spaces % 2;
                    }

                    self.output
                        .write(String::from(" ").repeat(spaces).as_bytes())
                        .expect("write error");
                    spaces = 0;


                    self.output.write(&[c as u8]).expect("write error");
                }
            };


            column += 1;
            let blank = c == b' ' || c == b'\t';
            convert &= self.all || blank;
        }

        self.output.write(b"\n").expect("write error");
        self.output.flush();
    }
}

fn main() {
    let yaml = load_yaml!("unexpand.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut unexpand = Unexpand::from_matches(&matches);
    let cwd = match current_dir() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("rm: error reading current working directory: {}", err);
            process::exit(1);
        }
    };

    let files: Vec<String> = match matches.values_of("FILE") {
        Some(files) => files
            .map(|file| {
                if file == "-" {
                    return String::from("-");
                }

                file.split_whitespace()
                    .map(|s| cwd.join(s.to_string()).to_str().unwrap().to_string())
                    .collect()
            })
            .collect(),
        None => vec!["-".to_string()],
    };

    for file_path in files {
        if file_path == "-" {
            let stdin = stdin();
            for line in stdin.lock().lines() {
                unexpand.unexpand_line(line.unwrap());
            }
        } else {
            let fd = File::open(file_path).unwrap();
            let reader = BufReader::new(fd);
            for line in reader.lines() {
                unexpand.unexpand_line(line.unwrap());
            }
        }
    }
}

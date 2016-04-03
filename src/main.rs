extern crate rustc_serialize;
extern crate getopts;

use std::env;
use std::collections::HashMap;
use rustc_serialize::json;
use getopts::Options;
use std::iter::repeat;

struct Opts {
    start: Coord,
    size: Coord,
    checkpoints: HashMap<i32, Coord>,
    help: bool,
}

impl Opts {
    pub fn new() -> Opts {
        Opts {
            start: Coord {x: 0, y:0},
            size: Coord {x: 6, y: 6},
            checkpoints: default_checkpoints(),
            help: false,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd, RustcDecodable)]
struct Coord {
    x: i16,
    y: i16,
}

pub trait UniqueVec<T> where T: Clone + Ord {
    fn upush(&mut self, item: T);
}

impl<'a, T> UniqueVec<T> for Vec<T> where T: Clone + Ord {
    fn upush(&mut self, item: T) {
        if !self.contains(&item) {
            self.push(item);
        }
    }
}

fn main() {
    let opts: Opts = parse_args();
    if opts.help {
        return;
    }

    let delta = [
        Coord{ x:  2, y:  1 },
        Coord{ x:  2, y: -1 },
        Coord{ x: -2, y:  1 },
        Coord{ x: -2, y: -1 },
        Coord{ x:  1, y: -2 },
        Coord{ x:  1, y:  2 },
        Coord{ x: -1, y:  2 },
        Coord{ x: -1, y: -2 },
    ];

    let mut paths = Vec::new();
    
    let pos: Coord = Coord { x: 0, y: 0 };

    println!("Starting journey at ({},{})", pos.x, pos.y);

    let mut path: Vec<Coord> = Vec::new();
    path.upush(opts.start.clone());

    find_path(&delta, &opts.size, &opts.start, &mut path, &mut paths, &opts.checkpoints);
}

/// Recursively look for all valid paths to tour the board from starting position
fn find_path(
    delta: &[Coord], 
    size: &Coord, 
    pos: &Coord, 
    mut path: &mut Vec<Coord>, 
    mut paths: &mut Vec<Vec<Coord>>,
    checkpoints: &HashMap<i32, Coord>
) {

    if path.len() as i16 >= size.x * size.y {
        paths.push(path.clone());
        print_path(&size, &path, paths.len());
        return;
    }

    let mut new_pos;
    for d in delta {
        new_pos = Coord { x: d.x + pos.x, y: d.y + pos.y};

        if !is_reachable(&new_pos, size, path.clone()) {
            continue;
        }

        if fails_checkpoint(&new_pos, path.clone(), &checkpoints) {
            continue;
        }

        path.upush(new_pos.clone());
        find_path(&delta, &size, &new_pos, &mut path, &mut paths, &checkpoints);
        path.pop();
    }
}

/// Print the board with corresponding move numbers
fn print_path(size: &Coord, path: &Vec<Coord>, path_num: usize) {
    let mut board: Vec<Vec<i32>> = (0..size.y).map(|_| vec![0; size.x as usize]).collect();
    board[0][0] = 1;
    let mut move_num = 0;

    for step in path {
        move_num += 1;
        board[step.y as usize][step.x as usize] = move_num;
    }

    println!("Journey {}", path_num);
    for x in board {
        println!("{}", repeat("-").take(31).collect::<String>());
        for y in x {
            print!("| {:02} ", y);
        }
        println!("|");
    }
    println!("{}", repeat("-").take(31).collect::<String>());
}

/// Check if the square has already been hit, or is out of bounds
fn is_reachable(pos: &Coord, size: &Coord, path: Vec<Coord>) -> bool {
    if pos.x < 0 || pos.y < 0 || pos.x >= size.x || pos.y >= size.y {
        return false;
    }

    if path.contains(&pos) {
        return false;
    }

    return true;
}

/// Creates a default checkpoints hashmap
///
/// These are specific points the knight should land on on a given move number
///
/// # Example
///
/// ```
/// checkpoints.insert(2, Coord { x: 1, y: 2 });
/// ```
/// Indicates that on move 2, the knight should land on coordinate (1,2)
///
fn default_checkpoints() -> HashMap<i32, Coord> {
    let mut checkpoints: HashMap<i32, Coord> = HashMap::new();

    checkpoints.insert(2, Coord { x: 1, y: 2 });
    checkpoints.insert(11, Coord { x: 0, y: 4 });
    checkpoints.insert(15, Coord { x: 5, y: 1 });
    checkpoints.insert(19, Coord { x: 1, y: 3 });
    checkpoints.insert(28, Coord { x: 5, y: 0 });

    checkpoints
}

/// Checks if the next move number is a checkpoint
/// If it is, it should validate against that checkpoint position
fn fails_checkpoint(pos: &Coord, path: Vec<Coord>, checkpoints: &HashMap<i32, Coord>) -> bool {
    let move_num = path.len() as i32 + 1;

    if checkpoints.contains_key(&move_num) {
        let checkpoint: &Coord = checkpoints.get(&move_num).unwrap();
        if pos.x != checkpoint.x || pos.y != checkpoint.y {
            return true;
        }
    }

    return false;
}

fn parse_args() -> Opts {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut options: Opts = Opts::new();

    let mut opts = Options::new();
    opts.optopt("s", "start", "The knight's starting position", "{\"x\": 0, \"y\": 0}");
    opts.optopt("d", "dimensions", "The dimensions of the board", "{\"x\": 6, \"y\": 6}");
    opts.optopt("c", "checkpoints", "JSON representation of positions the knight must hit on certain moves in the form {move_num: position}", "{\"2\": {\"x\": 2, \"y\": 3}}");
    opts.optflag("h", "help", "Print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return options;
    }

    if matches.opt_present("s") {
        options.start = match json::decode(&matches.opt_str("s").unwrap()) {
            Ok(j) => { j }
            Err(f) => { f.to_string(); return print_usage(&program, opts); }
        };
    }

    if matches.opt_present("d") {
        options.size = match json::decode(&matches.opt_str("d").unwrap()) {
            Ok(j) => { j }
            Err(f) => { f.to_string(); return print_usage(&program, opts); }
        };
    }

    if matches.opt_present("c") {
        options.checkpoints = match json::decode(&matches.opt_str("c").unwrap()) {
            Ok(j) => { j }
            Err(f) => { f.to_string(); return print_usage(&program, opts); }
        };
    }

    options
}

fn print_usage(program: &str, opts: Options) -> Opts {
    let mut options = Opts::new();
    options.help = true;

    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));

    options
}

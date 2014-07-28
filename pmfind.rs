#![feature(macro_rules)]
extern crate getopts;
use std::os;
mod pathmatch;

fn main()
{
    let args = os::args();
    let program = args.get(0).clone();
    let optspec: &[getopts::OptGroup] = [
        getopts::optopt("C", "dir", "change directory to this before starting", "dir"),
        getopts::optflag("h", "help", "print help and exit"),
        ];
    match getopts::getopts(args.tail(), optspec) {
        Ok(matches) => {
            if matches.opt_present("h") {
                println!("{}", getopts::usage(format!("Print subpaths of current directory that match given pathmatch() patterns\n\nUsage:\n\t{} [options] [pattern ...]", program).as_slice(), optspec));
            } else {
                pmfind(&matches);
            }
        },
        Err(x) => fail!(x.to_str()),
    }
}

fn pmfind(matches: &getopts::Matches)
{
    match matches.opt_str("C") {
        Some(dir) => if !os::change_dir(&Path::new(dir.clone())) { fail!("Can not chdir to: {}", dir); },
        None => {},
    }
    match std::io::fs::walk_dir(&Path::new("")) {
        Ok(mut dirs_iter) => {
            for path in dirs_iter {
                match path.as_str() {
                    Some(path_str) => {
                        let mut accept = (matches.free.len() == 0);
                        for pattern_string in matches.free.iter() {
                            if pattern_string.as_slice().starts_with("!") {
                                if pathmatch::pathmatch(pattern_string.as_slice().slice_from(1), path_str) {
                                    accept = false;
                                }
                            } else {
                                if pathmatch::pathmatch(pattern_string.as_slice(), path_str) {
                                    accept = true;
                                }
                            }
                        }
                        if accept { 
                            println!("{}", path.display()); 
                        }
                    },
                    None => {},
                }
            }
        },
        Err(res) => fail!("{}", res.to_str()),
    }
}

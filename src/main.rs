#![feature(plugin)]
#![feature(io)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
#[plugin] #[no_link] extern crate docopt_macros;
extern crate tclscan;

use std::old_io;
use std::string;
use docopt::Docopt;
use tclscan::rstcl;

docopt!(Args derive Show, "
Usage: tclscan check ( - | <path> )
       tclscan parsestr ( - | <script-str> )
");

pub fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let take_stdin = args.cmd__;
    let stdin = match take_stdin {
        true => old_io::stdin().read_to_string().unwrap(),
        false => string::String::new(),
    };
    match (args.cmd_check, args.cmd_parsestr, take_stdin) {
        (true, false, false) =>
            tclscan::scan_file(&args.arg_path[]),
        (true, false, true) =>
            tclscan::scan_script(&stdin[]),
        (false, true, false) =>
            println!("{:?}", rstcl::parse_command(args.arg_script_str.as_slice())),
        (false, true, true) =>
            println!("{:?}", rstcl::parse_command(stdin.as_slice())),
        _ =>
            panic!("Internal error, cannot handle args"),
    }
}

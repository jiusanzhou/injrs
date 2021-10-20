
use injrs::process_windows::*;
use injrs::inject_windows::*;
use injrs::evelate_windows::*;

const USAGE_HELP: &str = "
USAGE:
injrs PROCESS_NAME/PID [Libraies...]

EXAMPLES:
1. Inject test.dll to process Calc.exe
    $ injrs Calc.exe test.dll

2. Inject test.dll and demo.dll to process with PID: 1888
    $ injrs 1888 test.dll demo.dll
";

fn main() {
    println!("Welcome to have injrs. A library injector written by Rust.");
    // load args
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("{}", USAGE_HELP);
        return
    }
    let pid_or_name = args.nth(1).expect("must give a process name or pid");
    let dlls: Vec<String> = std::env::args().skip(2).collect();
    if dlls.len() == 0 {
        println!("You at least give one file to inject");
        return
    }

    let process: Process;
    match pid_or_name.parse::<u32>() {
        Ok(pid) => {
            // process pid
            match Process::from_pid(pid) {
                None => {
                    println!("can't find process with pid: {}", pid);
                    return
                },
                Some(p) => process = p
            }
        },
        Err(_) => {
            let name = pid_or_name.as_str();
            // process name
            match Process::find_first_by_name(name) {
                None => {
                    println!("can't find process with name: {}", name);
                    return
                },
                Some(p) => process = p
            }
        }
    }

    let _ = evelate_privileges();

    for i in dlls {
        print!("{} => ", i);
        match process.inject(i.as_str()) {
            Err(e) => {
                println!("error: {}", e);
            },
            Ok(_) => {
                println!("success");
            }
        }
    }

}
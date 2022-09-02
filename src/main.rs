use std::{env::args, thread::sleep, time::Duration, collections::HashMap};
use sysinfo::{System, SystemExt, CpuExt, CpuRefreshKind};

fn main() {
    let mut s: System = System::new();

    let mut args: Vec<String> = args().collect();
    let mut flags: HashMap<String, String> = HashMap::new();
    let mut devs: Vec<String> = vec![];

    // Iterates through arguments
    // Identifies wether it's a flag or a device
    // Appends the argument to corresponding variable
    args.remove(0); //removes first argument as it is the initial command
    while args.len() > 0 {
        if &args[0][0..1] == "-" {
            flags.insert(args.remove(0), args.remove(0));
        } else if args[0].len() > 2 {
            devs.push(args.remove(0))
        } else {
            eprintln!("Invalid argument: \x1b[31m'{}'\x1b[0m, arguments should either be a \x1b[33mflag\x1b[0m or a \x1b[33mstring\x1b[0m 3 characters long that represents a device", args.remove(0));
            break
        }
    }

    for dev in devs {
        match &dev[0..3] {
            "cpu"=>{
                //if cpu index is specified
                if dev.len() > 3 {
                    print!("{}", cpu(&mut s, Some(&dev[3..])))
                } else {
                    print!("{}", cpu(&mut s, None))
                }
            }
            _=>eprintln!("Device: \x1b[31m'{}'\x1b[0m not supported.", dev)
        }
    }
}

fn cpu(s: &mut System, core: Option<&str>) -> String {
    s.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    match core {
        Some(c)=>{
            let cpu_len: usize = s.cpus().len();
            let core_usize = c.parse::<usize>();

            let err_msg = format!("You only have \x1b[33m{}\x1b[0m cpus, pick one of your cpus from \x1b[33m0..{}\x1b[0m", cpu_len, cpu_len - 1);

            match core_usize {
                Ok(c)=>{
                    if c < cpu_len {
                        //prints specific cpu
                        return s.cpus()[c].cpu_usage().to_string()
                    } else {
                        return err_msg
                    }
                }
                Err(_e)=>err_msg
            }
        }
        None=>{
            //return average between all cpus
            return s.global_cpu_info().cpu_usage().to_string()
        }
    }
}

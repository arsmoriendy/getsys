use std::{env};
use std::collections::HashMap;

fn main() {
    //Parse device and flag arguments
    let mut flags: HashMap<String, String> = HashMap::new();
    let mut devs: Vec<String> = vec![];
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); //remove first argument
    while args.len() > 0 {
        if &args[0][0..1] == "-" {
            flags.insert(args.remove(0), args.remove(0));
        } else {
            devs.push(args.remove(0))
        }
    }

    for dev in devs {
        let dev_str = dev.as_str();
        match dev_str {
            "cpu"=>cpu(Some(flags.get("-c").unwrap_or(&"0".to_string()))),
            _=>eprintln!("Device: '{}' not supported.", dev_str)
        }
    }
}

fn cpu(core:Option<&str>) {
    use sysinfo::{System, SystemExt, CpuExt, CpuRefreshKind};
    let mut s = System::new();
    s.refresh_cpu_specifics(CpuRefreshKind::everything());
    s.refresh_cpu_specifics(CpuRefreshKind::everything());
    match core {
        Some(n)=>{
            let cpu_len: usize = s.cpus().len();
            let n_usize = n.parse::<usize>().unwrap();

            if n_usize < cpu_len {
                println!("{}", s.cpus()[n_usize].cpu_usage());
            } else {
                eprintln!("Select one of the 0..{} CPUs in your system, CPU {} doesn't exist!", cpu_len - 1, n_usize);
            }
        }
        None=>{
            println!("{}", s.cpus()[0].cpu_usage());
        }
    }
}

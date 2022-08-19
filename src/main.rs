use std::{env::args, thread::sleep, time::Duration, collections::HashMap};

fn main() {
    //Parse device and flag arguments
    let mut flags: HashMap<String, String> = HashMap::new();
    let mut devs: Vec<String> = vec![];
    let mut args: Vec<String> = args().collect();
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
            "cpu"=>cpu(flags.get("-c")),
            _=>eprintln!("Device: '{}' not supported.", dev_str)
        }
    }
}

fn cpu(core:Option<&String>) {
    use sysinfo::{System, SystemExt, CpuExt, CpuRefreshKind};
    let mut s = System::new();
    s.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    match core {
        Some(c)=>{
            let cpu_len: usize = s.cpus().len();
            let core_usize: usize = c.parse::<usize>().unwrap();

            if core_usize < cpu_len {
                //prints specific cpu
                loop {
                    println!("{}", s.global_cpu_info().cpu_usage());
                    s.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
                    sleep(Duration::from_secs(1));
                }
            } else {
                eprintln!("CPU {} doesn't exist, you must pick one of your CPUs between 0 to {}", core_usize, cpu_len - 1);
            }
        }
        None=>{
            //prints average between all cpus
            loop {
                println!("{}", s.global_cpu_info().cpu_usage());
                s.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
                sleep(Duration::from_secs(1));
            }
        }
    }
}

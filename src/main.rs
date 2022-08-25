use std::{env::args, thread::sleep, time::Duration, collections::HashMap};

fn main() {
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
        if dev.len() > 3 {
          cpu(Some(&dev[3..]))
        } else {cpu(None)}
      }
      _=>eprintln!("Device: \x1b[31m'{}'\x1b[0m not supported.", dev)
    }
  }
}

fn cpu(core:Option<&str>) {
  use sysinfo::{System, SystemExt, CpuExt, CpuRefreshKind};
  let mut s = System::new();
  s.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
  match core {
    Some(c)=>{
      let cpu_len: usize = s.cpus().len();
      let core_usize = c.parse::<usize>();

      fn argerr(cpu_len: usize) {
        eprintln!("You only have \x1b[33m{}\x1b[0m cpus, pick one of your cpus from \x1b[33m0..{}\x1b[0m", cpu_len, cpu_len - 1);
      }

      match core_usize {
        Ok(c)=>{
          if c < cpu_len {
            //prints specific cpu
            loop {
              println!("{}", s.cpus()[c].cpu_usage());
              s.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
              sleep(Duration::from_secs(1));
            } } else {argerr(cpu_len)}
        }
        Err(_e)=>argerr(cpu_len)
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

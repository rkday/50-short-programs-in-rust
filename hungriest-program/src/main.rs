use std::collections::HashMap;

fn main() {
    let mut original_times = HashMap::new();

    for prog in psutil::process::all().expect("Could not get all processes the first time") {
        original_times.insert(prog.pid, prog.utime + prog.stime);
    }

    std::thread::sleep(std::time::Duration::new(2,0));

    let mut busiest_process = (String::new(), 0.0);
    let mut most_ram = (String::new(), 0);

    for prog in psutil::process::all().expect("Could not get all processes the second time") {
        let rss = prog.memory().expect(&format!("Could not get memory for {}", prog.comm)).resident;
        if rss > most_ram.1 {
            most_ram = (prog.comm.clone(), rss);
        }
        if original_times.contains_key(&prog.pid) {
            let secs_passed = (prog.utime + prog.stime) - original_times[&prog.pid];
            if secs_passed > busiest_process.1 {
                busiest_process = (prog.comm, secs_passed);
            }

        }
        original_times.insert(prog.pid, prog.utime + prog.stime);
    }

    if busiest_process.0 == most_ram.0 {
        println!("{}: {:.1}%, {} MiB", busiest_process.0, (busiest_process.1/2.0)*100.0, most_ram.1/(1024*1024));
    } else {
        println!("{}: {:.1}% / {}: {} MiB", busiest_process.0, (busiest_process.1/2.0)*100.0, most_ram.0, most_ram.1/(1024*1024));
    }
}

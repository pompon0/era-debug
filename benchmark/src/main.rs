use clap::Parser;
use std::time;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    threads: usize,
    #[arg(long)]
    seconds: u64,
}

// ~0.1s
fn unit() -> usize {
    let p : u64 = 1000000007;
    let mut c : u64 = 1;
    let f = 7892342;
    for _ in 0..20000000 {
        c = c*f%p;
    }
    (c!=0) as usize
}

fn main() {
    let args = Args::parse();
    let deadline = time::Instant::now() + time::Duration::from_secs(args.seconds);
    std::thread::scope(|s| {
        let tasks : Vec<_> = (0..args.threads).map(|_|s.spawn(||{
            let mut res = 0;
            while time::Instant::now() < deadline {
                res += unit();
            }
            res
        })).collect();
        let mut cycles : u64 = 0;
        while time::Instant::now() < deadline {
            for i in 0.. {
                let Ok(freq) = std::fs::read_to_string(format!("/sys/devices/system/cpu/cpu{i}/cpufreq/scaling_cur_freq")) else {
                    break;
                };
                cycles += freq.trim().parse::<u64>().expect(&freq);
            }
            std::thread::sleep(time::Duration::from_millis(100));
        }
        let units : usize = tasks.into_iter().map(|t|t.join().unwrap()).sum();
        let cycles = cycles*1000/10; // * kHz/0.1s
        println!("units = {units}");
        println!("units/s = {}",(units as f64)/(args.seconds as f64));
        println!("units/thread = {:.2}",(units as f64)/(args.threads as f64));
        println!("units/thread/s = {:.2}",(units as f64)/(args.threads as f64)/(args.seconds as f64));
        println!("cycles = {cycles}");
        println!("cycles/s = {}",(cycles as f64)/(args.seconds as f64));
        println!("cycles/thread = {}",(cycles as f64)/(args.threads as f64));
        println!("cycles/thread/s = {}",(cycles as f64)/(args.threads as f64)/(args.seconds as f64));
        println!("cycles/unit = {}",(cycles as f64)/(units as f64));
    });
}

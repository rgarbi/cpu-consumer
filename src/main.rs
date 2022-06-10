use std::thread::sleep;
use std::time::{Duration, SystemTime};
use rand::Rng;
use systemstat::{System, Platform};



fn main() {
    let cpus = num_cpus::get();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(cpus + 2).build().unwrap();
    let mut thread_handles = Vec::new();


    for i in 1..500 {
        let handle = pool.spawn(move || {
            println!("Thread {}", i);
            let mut rng = rand::thread_rng();
            loop {
                let numerator: f64 = rng.gen();
                let denominator: f64 = rng.gen();
                let _answer = numerator / denominator;
                //println!("Thread {} - {}", i, answer);
            }
        });
        thread_handles.push(handle);
    }

    let now = SystemTime::now();
    loop {
        let sys = System::new();
        sleep(Duration::from_secs(1));
        println!("Been running for: {} seconds.", now.elapsed().unwrap().as_secs());
        match sys.cpu_load_aggregate() {
            Ok(cpu)=> {
                println!("\nMeasuring CPU load...");
                let cpu = cpu.done().unwrap();
                println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                         cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
            },
            Err(x) => println!("\nCPU load: error: {}", x)
        }

        match sys.cpu_temp() {
            Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
            Err(x) => println!("\nCPU temp: {}", x)
        }
    }
}

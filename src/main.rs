extern crate systemstat;

use std::thread;
use std::time::Duration;
use systemstat::{ByteSize, Platform, System};

fn main() {
    let sys = System::new();

    // Debug stats
    println!("Debug:");
    let bat_score = score_bat(&sys);
    let mem_score = score_mem(&sys);
    let cpu_score = score_cpu(&sys);
    let temp_score = temp_score(&sys);
    uptime(&sys);

    let mut scores = vec![mem_score, cpu_score];

    if let Some(score) = bat_score {
        scores.push(score);
    }

    if let Some(score) = temp_score {
        scores.push(score);
    }

    let avg_score = Score::avg(&scores);

    // Scores
    println!("\nScores:");
    println!("- Average Score:    {}", avg_score.to_string());

    if let Some(score) = bat_score {
        println!("- Battery Score:    {}", score.to_string());
    }

    println!("- Memory Score:     {}", mem_score.to_string());
    println!("- CPU Score:        {}", cpu_score.to_string());
    
    if let Some(score) = temp_score {
        println!("- CPU TEMP Score:   {}", score.to_string());
    }
}

// Simply print uptime
// TODO:  Score uptime
fn uptime(sys: &System) {
    match sys.uptime() {
        Ok(uptime) => println!("- Uptime: {:?}", uptime),
        Err(x) => println!("- Uptime: error: {}", x),
    }
}

// CPU temp may not be supported for some OSes or Systems
fn temp_score(sys: &System) -> Option<Score> {
    match sys.cpu_temp() {
        Ok(cpu_temp) => {
            let temp = cpu_temp as u8;
            println!("- CPU TEMP: {}", temp);
            return Some(Score::from_temp(temp));
        },
        Err(_) => None,
    }
}

fn score_bat(sys: &System) -> Option<Score> {
    match sys.battery_life() {
        Ok(bat) => {
            let bat = (bat.remaining_capacity * 100.0) as u8;
            println!("- BAT: {}", bat); // DEBUG INFO
            return Some(Score::from_percent(bat));
        }
        Err(_) => None,
    }
}

fn score_cpu(sys: &System) -> Score {
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            let cpu_idle = (cpu.idle * 100.0) as u8;
            println!("- CPU: {}", cpu_idle); // DEBUG INFO

            return Score::from_percent(cpu_idle);
        }
        Err(_) => panic!("Couldn't get CPU usage"),
    }
}

fn score_mem(sys: &System) -> Score {
    match sys.memory() {
        Ok(mem) => {
            println!(
                "- MEM: {} / {}",
                ByteSize::b(mem.total.as_u64() - mem.free.as_u64()),
                mem.total
            ); // DEBUG INFO
            let mem_usage = (mem.free.as_u64() * 100 / mem.total.as_u64()) as u8;
            return Score::from_percent(mem_usage as u8);
        }
        Err(_) => panic!("Couldn't get Memory usage."),
    }
}

#[derive(Clone, Copy)]
enum Score {
    Awful,
    Bad,
    Poor,
    Fair,
    Good,
    Perfect,
}

impl Score {
    fn from_percent(percent: u8) -> Self {
        return match percent {
            0..=9 => Score::Awful,
            10..=24 => Score::Bad,
            25..=49 => Score::Poor,
            50..=66 => Score::Fair,
            67..=89 => Score::Good,
            90..=100 => Score::Perfect,
            _ => panic!("Out of scoring range."),
        };
    }

    fn from_temp(temp: u8) -> Self {
        return match temp {
            0..=30 => Score::Perfect,
            31..=45 => Score::Good,
            46..=60 => Score::Fair,
            61..=75 => Score::Poor,
            76..=85 => Score::Bad,
            86..=110 => Score::Awful,
            _ => Score::Awful,
        }
    }

    fn from_int(int: usize) -> Self {
        return match int {
            0 => Score::Awful,
            1 => Score::Bad,
            2 => Score::Poor,
            3 => Score::Fair,
            4 => Score::Good,
            5 => Score::Perfect,
            _ => panic!("Unknown value: {}", int),
        };
    }

    fn to_string(self) -> String {
        return match self {
            Score::Awful => String::from("Awful"),
            Score::Bad => String::from("Bad"),
            Score::Poor => String::from("Poor"),
            Score::Fair => String::from("Fair"),
            Score::Good => String::from("Good"),
            Score::Perfect => String::from("Perfect"),
        };
    }

    fn avg(vec: &Vec<Self>) -> Self {
        let mut sum = 0;

        for u in vec {
            sum += *u as usize;
        }

        let avg = sum / vec.len();

        Score::from_int(avg as usize)
    }
}

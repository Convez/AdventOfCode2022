#![warn(clippy::all)]

struct CPU{
    probe_cycle: usize,
    curr_cycle: usize,
    execution_complete: bool,
    curr_command: String,
    command_amout: Option<i32>,
    command_cycle: usize,
    register: i32
}
struct CRT{
    curr_pixel: usize,
    screen_width: usize,
    screen_height: usize,
    pixels: Vec<String>
}

fn main(){
    let input = include_str!("input.txt");
    let debug = false;
    let mut lines = input.lines();
    
    let mut cpu = CPU{ probe_cycle: 20, curr_cycle: 0, execution_complete: true, curr_command: "".to_string(), command_amout: None, register:1, command_cycle: 0 };
    let mut crt = CRT{curr_pixel: 0, screen_height:6, screen_width:40, pixels:Default::default()};
    
    let mut probed_strengths : Vec<i32> = Default::default();
    for _ in 0..crt.screen_width*crt.screen_height{
        cpu.curr_cycle+=1;
        if debug{
            println!("Register value at cycle {} is {}. Signal strength is {}", cpu.curr_cycle, cpu.register,   cpu.curr_cycle as i32 * cpu.register);
        }
        if cpu.curr_cycle == cpu.probe_cycle && cpu.probe_cycle <= 220{
            let signal_strength = cpu.curr_cycle as i32 * cpu.register;
            println!("Register value at cycle {} is {}. Signal strength is {}", cpu.curr_cycle, cpu.register, signal_strength);
            probed_strengths.push(signal_strength);
            cpu.probe_cycle+=40;
        }
        if cpu.execution_complete{
            cpu.execution_complete = false;
            let command = lines.next().unwrap().trim();
            if command == "noop"{
                cpu.curr_command = command.to_string();
                cpu.command_amout = None;
                cpu.command_cycle = cpu.curr_cycle;
            }else{
                let amount = sscanf::sscanf!(command,"addx {i32}").unwrap();
                cpu.curr_command = "addx".to_string();
                cpu.command_amout = Some(amount);
                cpu.command_cycle = cpu.curr_cycle;
            }
        }
        // Draw CRT
        // Draw currnt pixel if register is visible
        if debug{
            println!("Register is {} and current pixel is {}. I can display if current pixel is {} or {} or {}",
                cpu.register,
                crt.curr_pixel,
                cpu.register-1,
                cpu.register,
                cpu.register+1
            );
        }
        if (cpu.register - 1 == crt.curr_pixel as i32) || (cpu.register == crt.curr_pixel as i32) || (cpu.register+1 ==crt.curr_pixel as i32){
            crt.pixels.push("#".to_string());
        }else{
            crt.pixels.push(".".to_string());
        }
        crt.curr_pixel+=1;
        crt.curr_pixel = crt.curr_pixel % crt.screen_width;
        // End the cycle
        if !cpu.execution_complete{
            if cpu.curr_command == "noop"{
                cpu.execution_complete = true;
            }else if cpu.curr_command == "addx" && cpu.command_cycle != cpu.curr_cycle{
                cpu.register += cpu.command_amout.unwrap();
                cpu.execution_complete = true;
            }
        }
    }
    println!("The sum of probed strengths is {}", probed_strengths.iter().sum::<i32>());
    for i in 0..crt.pixels.len(){
        print!("{}", crt.pixels[i]);
        if ((i+1) % crt.screen_width) == 0{
            println!("");
        }
    }
}

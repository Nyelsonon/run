use std::env;
use std::process::Command;
use open;
use colored::*;
use sysinfo::{ComponentExt, System, SystemExt};
extern crate num_cpus;


fn main() {
    let args = env::args().nth(1);
    let arg_store = args.clone().unwrap().to_string();
    if args.clone().unwrap().to_string() == "browse" {
        open::that("https://google.com").unwrap();
    }
    else if args.clone().unwrap().to_string() == "sysview" {
        ascii();
    } else if args.clone().unwrap().to_string()=="ip"{
        ascii();
        println!("Recieving Current Ip...");
        Command::new("ipconfig")
                .arg("getifaddr")
                .arg("en0")
                .spawn()
                .expect("failed to execute process");
        
    }else {
        println!("Running {}...", arg_store.clone());
        Command::new("open")
                .arg("-a")
                .arg(arg_store.clone())
                .output()
                .expect("failed to execute process");
    }
}


pub fn ascii() {
    let mut sys = sysinfo::System::new();
    sys.refresh_all();
    let num = num_cpus::get();
    println!("{}", "      ______________".red().bold());
    println!("{}        {}", "     /_____________/|".red().bold(), "<-----System Info----->".blue().bold());
    println!("{}", "    /_____________/ |".yellow().bold());
    println!("{}        {} {:?} KB", "   /____________ /  |".yellow().bold(), "Total Memory:".bold().cyan(), sys.total_memory());
    println!("{}        {} {:?} KB", "  | ___________ |   |".green().bold(), "Used Memory:".bold().cyan(), sys.used_memory());
    println!("{}{}{}        {} {:?} KB", "  ||     ".green().bold(),"/","     ||   |".green().bold(), "Total Swap:".bold().cyan(), sys.total_swap());
    println!("{}{}{}        {} {:?} KB", "  ||  ".blue().bold(),"/||||\\","   ||   |".blue().bold(), "Used Swap:".bold().cyan(),sys.used_swap());
    println!("{}{}{}        {} {:?}", "  ||  ".blue().bold(),"\\||||/","   ||   |".blue().bold(), "Name:".bold().cyan(),sys.name());
    println!("{}        {} {:?}", "  ||___________||   |".cyan().bold(), "Kernel Version:".bold().cyan(),sys.kernel_version());
    println!("{}        {} {:?}", "  ||           ||   |".cyan().bold(), "OS Version:".bold().cyan(),sys.os_version());
    println!("{}         {} {:?}", "  |   _______   |  /".purple().bold(), "Host Name:".bold().cyan(),sys.host_name());
    println!("{}         {} {:?}", "  /|  (_______)  | /".purple().bold(), "CPU Core Count:".bold().cyan(), num);
    println!("{}", " (  |_____________|/".red().bold());
    println!("{}        {}", "  \\".red().bold(), "                  <-----System Info----->".blue().bold());
    println!("{}", ".=======================.".black().bold());
    println!("{}", "| ::::::::::::::::  ::: |".black().bold());
    println!("{}", "| ::::::::::::::[]  ::: |".black().bold());
    println!("{}", "|   -----------     ::: |".black().bold());
    println!("{}", "`-----------------------'".black().bold());
}
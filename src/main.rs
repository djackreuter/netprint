use std::net::Ipv4Addr;

use sysinfo::{System, SystemExt, Pid, Process, ProcessExt};
use wmi::{WMIConnection, COMLibrary};
use serde::Deserialize;
use clap::Parser;


#[derive(Deserialize, Debug)]
#[serde(rename = "MSFT_NetTcpConnection")]
#[serde(rename_all = "PascalCase")]
struct ConnectionData {
    local_address: String,
    local_port: u16,
    remote_address: String,
    remote_port: u16,
    owning_process: u32
}

fn get_struct_data(connections: Vec<ConnectionData>) {
    for conn in connections {
        let remote_ip: Ipv4Addr = conn.remote_address.parse::<Ipv4Addr>().unwrap();

        if remote_ip.is_unspecified() || remote_ip.is_loopback() {
            continue;
        }
        println!("\n[+] PID: {}", conn.owning_process);
        println!("[+] Remote Addr: {}", remote_ip.to_string());
        println!("[+] Remote Port: {}", conn.remote_port);
        println!("[+] Local Address: {}", conn.local_address);
        println!("[+] Local Port: {}", conn.local_port);
    }
}

/// Print TCP data for processes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of process to search for. E.g., ir_agent.exe
    #[arg(short, long, default_value_t = String::new())]
    process: String
}


fn main() {

    let args = Args::parse();
    let mut sys: System = System::new_all();

    sys.refresh_all();

    let wmi_conn: WMIConnection = WMIConnection::with_namespace_path("ROOT\\StandardCIMv2", COMLibrary::new().unwrap()).unwrap();

    let results: Vec<ConnectionData> = wmi_conn.query().unwrap();
    let mut conn_vec: Vec<ConnectionData> = Vec::new();

    println!("{0: <25} | {1: <10} | {2: <25} | {3: <10} | {4: <25} | {5: <10}",
    "Process Name", "PID", "Local Address", "Local Port", "Remote Address ", "Remote Port");
    for data in results {
        let pid: usize = data.owning_process as usize;
        let process: &Process = sys.process(Pid::from(pid)).unwrap();

        // get process name from the pid's.
        if args.process != "" {
            if process.name().to_lowercase() == args.process.to_lowercase() {
                println!("{0: <25} | {1: <10} | {2: <25} | {3: <10} | {4: <25} | {5: <10}",
                    process.name(), data.owning_process, data.local_address, data.local_port, data.remote_address, data.remote_port
                );
                conn_vec.push(data);
            }
        } else {
                println!("{0: <25} | {1: <10} | {2: <25} | {3: <10} | {4: <25} | {5: <10}",
                process.name(), data.owning_process, data.local_address, data.local_port, data.remote_address, data.remote_port
            );
        }
    }
    if conn_vec.len() > 0 {
        get_struct_data(conn_vec);
    }
}

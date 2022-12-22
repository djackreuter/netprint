use wmi::{WMIConnection, COMLibrary};
use serde::Deserialize;


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

fn main() {
    let wmi_conn: WMIConnection = WMIConnection::with_namespace_path("ROOT\\StandardCIMv2", COMLibrary::new().unwrap()).unwrap();

    let results: Vec<ConnectionData> = wmi_conn.query().unwrap();

    println!("{0: <10} | {1: <15} | {2: <10} | {3: <15} | {4: <10}",
    "PID", "Local Address", "Local Port", "Remote Address ", "Remote Port");
    for res in results {
        println!("{0: <10} | {1: <15} | {2: <10} | {3: <15} | {4: <10}",
            res.owning_process, res.local_address, res.local_port, res.remote_address, res.remote_port
        );
    }
}

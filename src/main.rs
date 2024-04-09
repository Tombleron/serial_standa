mod command;

use std::time::Duration;

use clap::Parser;
use command::{home::HomeParameters, StandaGetSetCommand};

// #[repr(C, packed)]
// #[derive(Deserialize, Debug)]
// struct GENIResponse {
//     cmd: u32,
//     manufacturer: [i8; 16],
//     part_number: [i8; 24],
//     reserved: [u8; 24],
//     crc: u16,
// }

// #[repr(C, packed)]
// #[derive(Deserialize, Debug)]
// struct GPWRResponse {
//     cmd: u32,
//     hold_current: u8,
//     curr_reduct_delay: u16,
//     power_off_delay: u16,
//     currnet_set_time: u16,
//     power_flags: u8,
//     reserved: [u8; 6],
//     crc: u16,
// }

// #[repr(C, packed)]
// #[derive(Deserialize, Debug, Serialize)]
// struct MOVRRequest {
//     delta_position: i32,
//     u_delta_position: i16,
//     reserved: [u8; 6],
// }

// impl StandaCommand for MOVRRequest {
//     fn cmd_name() -> &'static str {
//         "movr"
//     }
// }

// #[repr(C, packed)]
// #[derive(Serialize, Debug)]
// struct SMOVRequest {
//     speed: u32,
//     u_speed: u8,
//     accel: u16,
//     decel: u16,
//     antiplay_speed: u32,
//     u_antiplay_speed: u8,
//     move_flags: u8,
//     reserved: [u8; 9],
// }

// impl StandaCommand for SMOVRequest {
//     fn cmd_name() -> &'static str {
//         "smov"
//     }
// }

// #[repr(C, packed)]
// #[derive(Serialize, Debug)]
// struct GMOVRequest {}

// impl StandaCommand for GMOVRequest {
//     const HAS_CRC: bool = false;

//     fn cmd_name() -> &'static str {
//         "gmov"
//     }
// }

// #[repr(C, packed)]
// #[derive(Deserialize, Debug)]
// struct GMOVResponse {
//     cmd: u32,
//     speed: u32,
//     u_speed: u8,
//     accel: u16,
//     decel: u16,
//     antiplay_speed: u32,
//     u_antiplay_speed: u8,
//     move_flags: u8,
//     reserved: [u8; 9],
//     crc: u16,
// }

// #[repr(C, packed)]
// #[derive(Serialize, Debug)]
// struct MOVERequest {
//     position: i32,
//     u_position: i16,
//     reserved: [u8; 6],
// }

// impl StandaCommand for MOVERequest {
//     fn cmd_name() -> &'static str {
//         "move"
//     }
// }

// #[repr(C, packed)]
// #[derive(Serialize, Debug)]
// struct GHOMRequest {}

// impl StandaCommand for GHOMRequest {
//     const HAS_CRC: bool = false;

//     fn cmd_name() -> &'static str {
//         "ghom"
//     }
// }

// #[repr(C, packed)]
// #[derive(Deserialize, Debug)]
// struct GHOMResponse {
//     cmd: u32,
//     fast_home: u32,
//     u_fast_home: u8,
//     slow_home: u32,
//     u_slow_home: u8,
//     home_delta: i32,
//     u_home_delta: i16,
//     home_flags: u16,
//     reserved: [u8; 9],
//     crc: u16,
// }

// #[repr(C, packed)]
// #[derive(Serialize, Debug)]
// struct SHOMRequest {
//     fast_home: u32,
//     u_fast_home: u8,
//     slow_home: u32,
//     u_slow_home: u8,
//     home_delta: i32,
//     u_home_delta: i16,
//     home_flags: u16,
//     reserved: [u8; 9],
// }

// impl StandaCommand for SHOMRequest {
//     fn cmd_name() -> &'static str {
//         "shom"
//     }
// }

// #[repr(C, packed)]
// #[derive(Serialize, Debug)]
// struct HOMERequest {}

// impl StandaCommand for HOMERequest {
//     const HAS_CRC: bool = false;
//     fn cmd_name() -> &'static str {
//         "home"
//     }
// }

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    device: String,
}

fn main() {
    let args = Args::parse();

    let port = serialport::new(args.device, 115_200)
        .timeout(Duration::from_secs(1))
        .open_native()
        .expect("Failed to open port");

    let home_params = HomeParameters::get(port);

    println!("{:#?}", home_params);
}

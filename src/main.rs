mod command;

use std::{
    io::{Read, Write},
    mem::size_of,
    time::Duration,
};

use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Deserialize, Serialize};





// struct StandaDevice {
//    home_parameters: HomeParameters,
// }



#[repr(C, packed)]
#[derive(Deserialize, Debug)]
struct GENIResponse {
    cmd: u32,
    manufacturer: [i8; 16],
    part_number: [i8; 24],
    reserved: [u8; 24],
    crc: u16,
}

#[repr(C, packed)]
#[derive(Deserialize, Debug)]
struct GPWRResponse {
    cmd: u32,
    hold_current: u8,
    curr_reduct_delay: u16,
    power_off_delay: u16,
    currnet_set_time: u16,
    power_flags: u8,
    reserved: [u8; 6],
    crc: u16,
}

#[repr(C, packed)]
#[derive(Deserialize, Debug, Serialize)]
struct MOVRRequest {
    delta_position: i32,
    u_delta_position: i16,
    reserved: [u8; 6],
}

// impl StandaCommand for MOVRRequest {
//     fn cmd_name() -> &'static str {
//         "movr"
//     }
// }

#[repr(C, packed)]
#[derive(Serialize, Debug)]
struct SMOVRequest {
    speed: u32,
    u_speed: u8,
    accel: u16,
    decel: u16,
    antiplay_speed: u32,
    u_antiplay_speed: u8,
    move_flags: u8,
    reserved: [u8; 9],
}

// impl StandaCommand for SMOVRequest {
//     fn cmd_name() -> &'static str {
//         "smov"
//     }
// }

#[repr(C, packed)]
#[derive(Serialize, Debug)]
struct GMOVRequest {}

// impl StandaCommand for GMOVRequest {
//     const HAS_CRC: bool = false;

//     fn cmd_name() -> &'static str {
//         "gmov"
//     }
// }

#[repr(C, packed)]
#[derive(Deserialize, Debug)]
struct GMOVResponse {
    cmd: u32,
    speed: u32,
    u_speed: u8,
    accel: u16,
    decel: u16,
    antiplay_speed: u32,
    u_antiplay_speed: u8,
    move_flags: u8,
    reserved: [u8; 9],
    crc: u16,
}

#[repr(C, packed)]
#[derive(Serialize, Debug)]
struct MOVERequest {
    position: i32,
    u_position: i16,
    reserved: [u8; 6],
}

// impl StandaCommand for MOVERequest {
//     fn cmd_name() -> &'static str {
//         "move"
//     }
// }

#[repr(C, packed)]
#[derive(Serialize, Debug)]
struct GHOMRequest {}

// impl StandaCommand for GHOMRequest {
//     const HAS_CRC: bool = false;

//     fn cmd_name() -> &'static str {
//         "ghom"
//     }
// }

#[repr(C, packed)]
#[derive(Deserialize, Debug)]
struct GHOMResponse {
    cmd: u32,
    fast_home: u32,
    u_fast_home: u8,
    slow_home: u32,
    u_slow_home: u8,
    home_delta: i32,
    u_home_delta: i16,
    home_flags: u16,
    reserved: [u8; 9],
    crc: u16,
}

#[repr(C, packed)]
#[derive(Serialize, Debug)]
struct SHOMRequest {
    fast_home: u32,
    u_fast_home: u8,
    slow_home: u32,
    u_slow_home: u8,
    home_delta: i32,
    u_home_delta: i16,
    home_flags: u16,
    reserved: [u8; 9],
}

// impl StandaCommand for SHOMRequest {
//     fn cmd_name() -> &'static str {
//         "shom"
//     }
// }

#[repr(C, packed)]
#[derive(Serialize, Debug)]
struct HOMERequest {}

// impl StandaCommand for HOMERequest {
//     const HAS_CRC: bool = false;
//     fn cmd_name() -> &'static str {
//         "home"
//     }
// }

fn main() {
    // let ports = serialport::available_ports().expect("No ports found!");
    // for p in ports {
    //     println!("{}", p.port_name);
    // }

    // println!("{}", crc16(&[1, 2, 3, 4]));

    // let mut port = serialport::new("/dev/ttyACM0", 115_200)
    //     .timeout(Duration::from_secs(1))
    //     .open_native()
    //     .expect("Failed to open port");

    // let req = SHOMRequest {
    //     fast_home: 20,
    //     u_fast_home: 0,
    //     slow_home: 20,
    //     u_slow_home: 0,
    //     home_delta: 160,
    //     u_home_delta: 0,
    //     home_flags: 250,
    //     reserved: [0; 9],
    // };

    // let req = HOMERequest {};

    // // let req = GHOMRequest {};
    // let input = req.as_bytes();
    // println!("Serialized: {:x?}", input);
    // port.write_all(&input).expect("Failed to write");
    // // port.write
    // //
    // println!("Sent");

    // let mut serial_buf: Vec<u8> = vec![0; 33];
    // port.read_exact(serial_buf.as_mut_slice())
    //     .expect("Found no data!");

    // println!("{:?}", std::str::from_utf8(&serial_buf));

    // let geni_resp = deserialize::<GHOMResponse>(&serial_buf).unwrap();
    // println!("{:#?}", geni_resp);
}


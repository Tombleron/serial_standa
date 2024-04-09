use std::io;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serialport::TTYPort;

use super::{StandaCommand, StandaGetSetCommand};

bitflags! {
    // #[repr(C, packed)]
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    #[serde(transparent)]
    pub struct MoveState: u8 {
        const MOVING = 0x1;
        const TARGET_SPEED = 0x2;
        const ANTIPLAY = 0x4;
    }
}

bitflags! {
    // #[repr(C, packed)]
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    #[serde(transparent)]
    pub struct MvCmdSts: u8 {
        const UNKNWN = 0x0;
        const MOVE = 0x1;
        const MOVR = 0x2;
        const LEFT = 0x3;
        const RIGHT = 0x4;
        const STOP = 0x5;
        const HOME = 0x6;
        const LOFT = 0x7;
        const SSTP = 0x8;
        const ERROR = 0x40;
        const RUNNING = 0x80;
    }
}

bitflags! {
    // #[repr(C, packed)]
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    #[serde(transparent)]
    pub struct PowerState: u8 {
        const UNKNOWN = 0x0;
        const OFF = 0x1;
        const NORM = 0x3;
        const REDUCT = 0x4;
        const MAX = 0x5;
    }
}

bitflags! {
    // #[repr(C, packed)]
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    #[serde(transparent)]
    pub struct EncoderState: u8 {
        const ABSENT = 0x0;
        const UNKNOWN = 0x1;
        const MALFUNC = 0x2;
        const REVERS = 0x3;
        const OK = 0x4;
    }
}

bitflags! {
    // #[repr(C, packed)]
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    #[serde(transparent)]
    pub struct WindState: u8 {
        const WIND_A_STATE_ABSENT = 0x0;
        const WIND_A_STATE_UNKNOWN = 0x1;
        const WIND_A_STATE_MALFUNC = 0x2;
        const WIND_A_STATE_OK = 0x3;
        const WIND_B_STATE_ABSENT = 0x0;
        const WIND_B_STATE_UNKNOWN = 0x10;
        const WIND_B_STATE_MALFUNC = 0x20;
        const WIND_B_STATE_OK = 0x30;
    }
}

bitflags! {
    // #[repr(C, packed)]
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    #[serde(transparent)]
    pub struct State: u32 {
        const ERRC = 0x1;
        const ERRD = 0x2;
        const ERRV = 0x4;
        const EPROM_CONNECTED = 0x10;
        const IS_HOMED = 0x20;
        const ALARM = 0x40;
        const CTP_ERROR = 0x80;
        const POWER_OVERHEAT = 0x100;
        const CONTROLLER_OVERHEAT = 0x200;
        const OVERLOAD_POWER_VOLTAGE = 0x400;
        const OVERLOAD_POWER_CURRENT = 0x800;
        const OVERLOAD_USB_VOLTAGE = 0x1000;
        const LOW_USB_VOLTAGE = 0x2000;
        const OVERLOAD_USB_CURRENT = 0x4000;
        const BORDERS_SWAP_MISSET = 0x8000;
        const LOW_POWER_VOLTAGE = 0x10000;
        const H_BRIDGE_FAULT = 0x20000;
        const WINDING_RES_MISMATCH = 0x100000;
        const ENCODER_FAULT = 0x200000;
        const ENGINE_RESPONSE_ERROR = 0x800000;
        const EXTIO_ALARM = 0x1000000;
    }
}

bitflags! {
    // #[repr(C, packed)]
    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    #[serde(transparent)]
    pub struct GpioFlags: u32 {
        const STATE_RIGHT_EDGE = 0x1;
        const STATE_LEFT_EDGE = 0x2;
        const STATE_BUTTON_RIGHT = 0x4;
        const STATE_BUTTON_LEFT = 0x8;
        const STATE_GPIO_PINOUT = 0x10;
        const STATE_GPIO_LEVEL = 0x20;
        const STATE_BRAKE = 0x200;
        const STATE_REV_SENSOR = 0x400;
        const STATE_SYNC_INPUT = 0x800;
        const STATE_SYNC_OUTPUT = 0x1000;
        const STATE_ENC_A = 0x2000;
        const STATE_ENC_B = 0x4000;
    }
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
pub struct StateParams {
    pub move_sts: MoveState,
    pub mv_cmd_sts: MvCmdSts,
    pub pwr_sts: PowerState,
    pub end_sts: EncoderState,
    pub wind_sts: WindState,

    pub cur_position: i32,
    pub u_cur_position: i16,

    pub enc_position: i32,
    pub u_enc_position: i16,

    pub cur_speed: i32,
    pub u_cur_speed: i16,

    pub i_pwr: i16,
    pub u_pwr: i16,
    pub i_usb: i16,
    pub u_usb: i16,

    pub cur_t: i16,

    pub state: State,
    pub gpio_flags: GpioFlags,

    pub cmd_buf_free_space: u8,
}

impl<'a> StandaCommand<'a> for StateParams {
    const RESERVED_BYTES: &'a [u8] = &[0; 4];
}

impl<'a> StandaGetSetCommand<'a> for StateParams {
    const GET_CMD_NAME: &'static str = "gets";
    const SET_CMD_NAME: &'static str = "";

    fn set(&self, _: &mut TTYPort) -> io::Result<()> {
        Ok(())
    }
}

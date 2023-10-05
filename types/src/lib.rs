use rgb::RGB;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[macro_export]
macro_rules! export {
    ($struct_name:ident) => {
        $struct_name::export_to(&format!(
            "./frontend/src/types/generated/{}.d.ts",
            stringify!($struct_name)
        ))?;
    };
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(tag = "type", content = "content")]
pub enum MessageType {
    Command(CommandType),
    Data(DataType),
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(tag = "type", content = "content")]
pub enum DataType {
    LedData(Vec<Pixel>),
}

#[derive(Serialize, Deserialize, TS, Debug, Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Pixel {
    pub fn rainbow_tick(&mut self) {
        const MAX: u8 = 20;
        const INCREMENT_LIMIT: u8 = MAX - 1;
        match (self.r, self.g, self.b) {
            (MAX, 0..=INCREMENT_LIMIT, 0) => {
                self.g += 1;
            }
            (1..=MAX, MAX, 0) => {
                self.r -= 1;
            }
            (0, MAX, 0..=INCREMENT_LIMIT) => {
                self.b += 1;
            }
            (0, 1..=MAX, MAX) => {
                self.g -= 1;
            }
            (0..=INCREMENT_LIMIT, 0, MAX) => {
                self.r += 1;
            }
            (MAX, 0, 1..=MAX) => {
                self.b -= 1;
            }
            _ => {
                self.r = MAX;
                self.g = 0;
                self.b = 0;
            }
        }
    }
}

impl From<RGB<u8>> for Pixel {
    fn from(value: RGB<u8>) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl From<Pixel> for RGB<u8> {
    fn from(value: Pixel) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(tag = "type")]
pub enum CommandType {
    GetLedData,
    Reboot,
}

use std::fmt::Display;

use egui::ComboBox;
use memscan::search::MemorySearch;
use serde::de::value;

use crate::{app_error::AppError, endian::Endianness};

#[derive(Debug, PartialEq, Default, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum DataType {
    U8,
    U16,
    U32,
    #[default]
    U64,
    // S8,
    // S16,
    // S32,
    // S64,
    F32,
    F64,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DataType {
    pub fn picker_for(&mut self, ui: &mut egui::Ui) {
        ComboBox::from_label("Datatype")
            .selected_text(format!("{:?}", self))
            .show_ui(ui, |ui| {
                ui.selectable_value(self, Self::U8, "u8");
                ui.selectable_value(self, Self::U16, "u16");
                ui.selectable_value(self, Self::U32, "u32");
                ui.selectable_value(self, Self::U64, "u64");
                // ui.selectable_value(self, Self::S8, "s8");
                // ui.selectable_value(self, Self::S16, "s16");
                // ui.selectable_value(self, Self::S32, "s32");
                // ui.selectable_value(self, Self::S64, "s64");
                ui.selectable_value(self, Self::F32, "f32");
                ui.selectable_value(self, Self::F64, "f64");
            });
    }

    pub fn size(self) -> usize {
        match self {
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::U64 => 8,
            // Self::S8 => 1,
            // Self::S16 => 2,
            // Self::S32 => 4,
            // Self::S64 => 8,
            Self::F32 => 4,
            Self::F64 => 8,
        }
    }

    pub fn parse(self, label: &str) -> Result<WrappedValue, AppError> {
        let result = match self {
            Self::U8 => WrappedValue::U8(label.parse()?),
            Self::U16 => WrappedValue::U16(label.parse()?),
            Self::U32 => WrappedValue::U32(label.parse()?),
            Self::U64 => WrappedValue::U64(label.parse()?),
            // Self::S8 => WrappedValue::S8(label.parse()?),
            // Self::S16 => WrappedValue::S16(label.parse()?),
            // Self::S32 => WrappedValue::S32(label.parse()?),
            // Self::S64 => WrappedValue::S64(label.parse()?),
            Self::F32 => WrappedValue::F32(label.parse()?),
            Self::F64 => WrappedValue::F64(label.parse()?),
        };

        Ok(result)
    }

    pub fn cast(self, buffer: &[u8], endianness: Endianness) -> Result<WrappedValue, AppError> {
        if buffer.len() < self.size() {
            return Err(AppError::DataTypeParseError(format!(
                "Buffer too small for {:?}",
                self
            )));
        }
        match endianness {
            Endianness::Little => {
                panic!("Big endian not implemented yet");
            }
            Endianness::Big => {
                panic!("Big endian not implemented yet");
            }
            Endianness::Native => {
                match self {
                    Self::U8 => Ok(WrappedValue::U8(buffer[0])),
                    Self::U16 => Ok(WrappedValue::U16(u16::from_ne_bytes(buffer.try_into()?))),
                    Self::U32 => Ok(WrappedValue::U32(u32::from_ne_bytes(buffer.try_into()?))),
                    Self::U64 => Ok(WrappedValue::U64(u64::from_ne_bytes(buffer.try_into()?))),
                    // Self::S8 => Ok(WrappedValue::S8(buffer[0])),
                    // Self::S16 => Ok(WrappedValue::S16(i16::from_ne_bytes(buffer.try_into().unwrap()))),
                    // Self::S32 => Ok(WrappedValue::S32(i32::from_ne_bytes(buffer.try_into().unwrap()))),
                    // Self::S64 => Ok(WrappedValue::S64(i64::from_ne_bytes(buffer.try_into().unwrap()))),
                    Self::F32 => Ok(WrappedValue::F32(f32::from_ne_bytes(buffer.try_into()?))),
                    Self::F64 => Ok(WrappedValue::F64(f64::from_ne_bytes(buffer.try_into()?))),
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WrappedValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    // S8(i8),
    // S16(i16),
    // S32(i32),
    // S64(i64),
    F32(f32),
    F64(f64),
}

impl WrappedValue {
    pub fn data_type(self) -> DataType {
        match self {
            WrappedValue::U8(_) => DataType::U8,
            WrappedValue::U16(_) => DataType::U16,
            WrappedValue::U32(_) => DataType::U32,
            WrappedValue::U64(_) => DataType::U64,
            // WrappedValue::S8(_) => DataType::S8,
            // WrappedValue::S16(_) => DataType::S16,
            // WrappedValue::S32(_) => DataType::S32,
            // WrappedValue::S64(_) => DataType::S64,
            WrappedValue::F32(_) => DataType::F32,
            WrappedValue::F64(_) => DataType::F64,
        }
    }

    pub fn compare_to(self, buffer: &[u8], endianness: Endianness) -> bool {
        match endianness {
            Endianness::Little => match self {
                WrappedValue::U8(value) => buffer[0] == value,
                WrappedValue::U16(value) => buffer[0..2] == value.to_le_bytes(),
                WrappedValue::U32(value) => buffer[0..4] == value.to_le_bytes(),
                WrappedValue::U64(value) => buffer[0..8] == value.to_le_bytes(),
                // WrappedValue::S8(value) => buffer[0] == value as u8,
                // WrappedValue::S16(value) => buffer[0..2] == value.to_le_bytes(),
                // WrappedValue::S32(value) => buffer[0..4] == value.to_le_bytes(),
                // WrappedValue::S64(value) => buffer[0..8] == value.to_le_bytes(),
                WrappedValue::F32(value) => buffer[0..4] == value.to_le_bytes(),
                WrappedValue::F64(value) => buffer[0..8] == value.to_le_bytes(),
            },
            Endianness::Big => match self {
                WrappedValue::U8(value) => buffer[0] == value,
                WrappedValue::U16(value) => buffer[0..2] == value.to_be_bytes(),
                WrappedValue::U32(value) => buffer[0..4] == value.to_be_bytes(),
                WrappedValue::U64(value) => buffer[0..8] == value.to_be_bytes(),
                // WrappedValue::S8(value) => buffer[0] == value as u8,
                // WrappedValue::S16(value) => buffer[0..2] == value.to_be_bytes(),
                // WrappedValue::S32(value) => buffer[0..4] == value.to_be_bytes(),
                // WrappedValue::S64(value) => buffer[0..8] == value.to_be_bytes(),
                WrappedValue::F32(value) => buffer[0..4] == value.to_be_bytes(),
                WrappedValue::F64(value) => buffer[0..8] == value.to_be_bytes(),
            },
            Endianness::Native => match self {
                WrappedValue::U8(value) => buffer[0] == value,
                WrappedValue::U16(value) => buffer[0..2] == value.to_ne_bytes(),
                WrappedValue::U32(value) => buffer[0..4] == value.to_ne_bytes(),
                WrappedValue::U64(value) => buffer[0..8] == value.to_ne_bytes(),
                // WrappedValue::S8(value) => buffer[0] == value as u8,
                // WrappedValue::S16(value) => buffer[0..2] == value.to_ne_bytes(),
                // WrappedValue::S32(value) => buffer[0..4] == value.to_ne_bytes(),
                // WrappedValue::S64(value) => buffer[0..8] == value.to_ne_bytes(),
                WrappedValue::F32(value) => buffer[0..4] == value.to_ne_bytes(),
                WrappedValue::F64(value) => buffer[0..8] == value.to_ne_bytes(),
            },
        }
    }

    pub fn scan_memory<'a>(
        self,
        buffer: &'a [u8],
        endianness: Endianness,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        match self {
            WrappedValue::U8(needle) => Box::new(MemorySearch::new(needle, buffer)),
            WrappedValue::U16(needle) => Box::new(MemorySearch::new(needle, buffer)),
            WrappedValue::U32(needle) => Box::new(MemorySearch::new(needle, buffer)),
            WrappedValue::U64(needle) => Box::new(MemorySearch::new(needle, buffer)),
            // WrappedValue::S8(needle) => Box::new(MemorySearch::new(needle, buffer)),
            // WrappedValue::S16(needle) => Box::new(MemorySearch::new(needle, buffer)),
            // WrappedValue::S32(needle) => Box::new(MemorySearch::new(needle, buffer)),
            // WrappedValue::S64(needle) => Box::new(MemorySearch::new(needle, buffer)),
            WrappedValue::F32(needle) => Box::new(MemorySearch::new(needle, buffer)),
            WrappedValue::F64(needle) => Box::new(MemorySearch::new(needle, buffer)),
        }
    }
}

impl Display for WrappedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WrappedValue::U8(value) => write!(f, "{}", value),
            WrappedValue::U16(value) => write!(f, "{}", value),
            WrappedValue::U32(value) => write!(f, "{}", value),
            WrappedValue::U64(value) => write!(f, "{}", value),
            // WrappedValue::S8(value) => write!(f, "{}", value),
            // WrappedValue::S16(value) => write!(f, "{}", value),
            // WrappedValue::S32(value) => write!(f, "{}", value),
            // WrappedValue::S64(value) => write!(f, "{}", value),
            WrappedValue::F32(value) => write!(f, "{}", value),
            WrappedValue::F64(value) => write!(f, "{}", value),
        }
    }
}

use std::str::FromStr;
use thiserror::Error as ThisError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Band {
    Meter2190,
    Meter630,
    Meter560,
    Meter160,
    Meter80,
    Meter40,
    Meter20,
    Meter17,
    Meter15,
    Meter12,
    Meter10,
    Meter8,
    Meter6,
    Meter5,
    Meter4,
    Meter2,
    Meter1P25,
    Centimeter70,
    Centimeter33,
    Centimeter23,
    Centimeter13,
    Centimeter9,
    Centimeter6,
    Centimeter3,
    Centimeter1P25,
    Millimeter6,
    Millimeter4,
    Millimeter2P5,
    Millimeter2,
    Millimeter1,
    SubMillimeter,
}

#[derive(Debug, Clone, ThisError)]
#[error("invalid band specifier")]
pub struct InvalidBand;

impl FromStr for Band {
    type Err = InvalidBand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2190m" => Ok(Band::Meter2190),
            "630m" => Ok(Band::Meter630),
            "560m" => Ok(Band::Meter560),
            "160m" => Ok(Band::Meter160),
            "80m" => Ok(Band::Meter80),
            "40m" => Ok(Band::Meter40),
            "20m" => Ok(Band::Meter20),
            "17m" => Ok(Band::Meter17),
            "15m" => Ok(Band::Meter15),
            "12m" => Ok(Band::Meter12),
            "10m" => Ok(Band::Meter10),
            "8m" => Ok(Band::Meter8),
            "6m" => Ok(Band::Meter6),
            "5m" => Ok(Band::Meter5),
            "4m" => Ok(Band::Meter4),
            "2m" => Ok(Band::Meter2),
            "1.25m" => Ok(Band::Meter1P25),
            "70cm" => Ok(Band::Centimeter70),
            "33cm" => Ok(Band::Centimeter33),
            "23cm" => Ok(Band::Centimeter23),
            "13cm" => Ok(Band::Centimeter13),
            "9cm" => Ok(Band::Centimeter9),
            "6cm" => Ok(Band::Centimeter6),
            "3cm" => Ok(Band::Centimeter3),
            "1.25cm" => Ok(Band::Centimeter1P25),
            "6mm" => Ok(Band::Millimeter6),
            "4mm" => Ok(Band::Millimeter4),
            "2.5mm" => Ok(Band::Millimeter2P5),
            "2mm" => Ok(Band::Millimeter2),
            "1mm" => Ok(Band::Millimeter1),
            "submm" => Ok(Band::SubMillimeter),
            _ => Err(InvalidBand),
        }
    }
}

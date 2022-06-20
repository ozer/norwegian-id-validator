use std::error::Error;
use std::{
    fmt::{Display, Formatter},
    u8,
};

const NORWEGIAN_ID_LENGTH: usize = 11;

const STATIC_SEQUENCE_FIRST_CHECK_DIGIT: &'static [u8; 10] = &[3, 7, 6, 1, 8, 9, 4, 5, 2, 1];
const STATIC_SEQUENCE_SECOND_CHECK_DIGIT: &'static [u8; 11] = &[5, 4, 3, 2, 7, 6, 5, 4, 3, 2, 1];

type NorwegianIdBytes = [u8; NORWEGIAN_ID_LENGTH];

#[derive(Debug)]
pub struct NorwegianId(NorwegianIdBytes);

#[derive(PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum IdType {
    BirthNumber,
    DNumber,
    HNumber,
    FHNumber,
}

impl From<NorwegianIdBytes> for NorwegianId {
    fn from(value: NorwegianIdBytes) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Debug, PartialEq)]
pub enum NorwegianIdValidationError {
    InvalidLength,
    NonNumericValue,
    InvalidChecksum,
}

impl std::fmt::Display for NorwegianIdValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Invalid length."),
            Self::NonNumericValue => write!(f, "Includes a non-numeric value."),
            Self::InvalidChecksum => write!(f, "Invalid checksum."),
        }
    }
}

impl Error for NorwegianIdValidationError {}

impl NorwegianId {
    pub fn parse(id: &str) -> Result<Self, NorwegianIdValidationError> {
        return is_valid(id)
        .map_err(|err| err)
        .map(|_| {
            let mut norwegian_id_bytes: NorwegianIdBytes = Default::default();
            let digits: Vec<u8> = id.as_bytes().iter().map(|digit| (digit - b'0')).collect();
            norwegian_id_bytes.copy_from_slice(&digits);

            NorwegianId(norwegian_id_bytes)
        })   
    }

    pub fn is_male(&self) -> bool {
        self.get_gender() == Gender::Male
    }

    pub fn is_female(&self) -> bool {
        self.get_gender() == Gender::Female
    }

    fn get_gender(&self) -> Gender {
        let char_at_9 = self.0.get(8).unwrap();
        return if char_at_9 % 2 == 0 {
            Gender::Female
        } else {
            Gender::Male
        };
    }

    pub fn get_id_type(&self) -> IdType {
        let first_digit = self.0[0];

        if first_digit == 8 || first_digit == 9 {
            return IdType::FHNumber;
        }

        if first_digit >= 4 && first_digit <= 7 {
            return IdType::DNumber;
        }

        let third_digit = self.0.get(2).unwrap();

        if third_digit == &4 || third_digit == &5 {
            return IdType::HNumber;
        }

        return IdType::BirthNumber;
    }
}

pub fn is_valid(id: &str) -> Result<bool, NorwegianIdValidationError> {
    if id.len() != 11 {
        return Err(NorwegianIdValidationError::InvalidLength);
    }

    let mut all_numeric = true;

    for c in id.chars() {
        if !c.is_numeric() {
            all_numeric = false;
        }
    }

    if !all_numeric {
        return Err(NorwegianIdValidationError::NonNumericValue);
    }

    let digit_vec: Vec<u8> = id.as_bytes().iter().map(|digit| (digit - b'0')).collect();

    let mut norwegian_id_bytes: NorwegianIdBytes = Default::default();

    norwegian_id_bytes.copy_from_slice(&digit_vec);

    let mut first_check_digit_sum = 0;

    for (pos, e) in STATIC_SEQUENCE_FIRST_CHECK_DIGIT.iter().enumerate() {
        first_check_digit_sum += e * norwegian_id_bytes.get(pos).unwrap();
    }

    let mut second_check_digit_sum = 0;
    for (pos, e) in STATIC_SEQUENCE_SECOND_CHECK_DIGIT.iter().enumerate() {
        second_check_digit_sum += e * norwegian_id_bytes.get(pos).unwrap();
    }

    if first_check_digit_sum % 11 == 0 && second_check_digit_sum % 11 == 0 {
        return Ok(true);
    } else {
        return Err(NorwegianIdValidationError::InvalidChecksum);
    }
}

impl Display for NorwegianId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}
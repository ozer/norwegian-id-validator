mod test_input;

use test_input::{
    NUMBERS_WITH_INVALID_CHECKSUM, NUMBERS_WITH_INVALID_LENGTH,
    VALID_BIRTH_NUMBERS_NORSK_HELSENETT_TESTAKTOERER,
    VALID_DNUMBER_FROM_NORSK_HELSENETT_TESTAKTOERER, VALID_D_NUMBERS, VALID_FEMALE_BIRTH_NUMBER,
    VALID_FHNUMBER_FROM_NORSK_HELSENETT_TESTAKTOERER,
    VALID_HNUMBER_FROM_NORSK_HELSENETT_TESTAKTOERER, VALID_MALE_BIRTH_NUMBER,
};

use norwegian_id_validator::{IdType, NorwegianId, NorwegianIdValidationError};

#[test]
fn test_valid_d_numbers() {
    for number in VALID_D_NUMBERS {
        let norwegian_id = NorwegianId::parse(number);

        assert_eq!(norwegian_id.is_ok(), true);
        assert_eq!(norwegian_id.unwrap().get_id_type(), IdType::DNumber);
    }
}

#[test]
fn test_valid_male_id() {
    for number in VALID_MALE_BIRTH_NUMBER {
        let norwegian_id_parse_result = NorwegianId::parse(number);
        assert_eq!(norwegian_id_parse_result.is_ok(), true);

        let norwegian_id = norwegian_id_parse_result.unwrap();


        assert_eq!(norwegian_id.get_id_type(), IdType::BirthNumber);
        assert_eq!(norwegian_id.is_male(), true);
    }
}

#[test]
fn test_valid_female_id() {
    for number in VALID_FEMALE_BIRTH_NUMBER {
        let norwegian_id_parse_result = NorwegianId::parse(number);
        assert_eq!(norwegian_id_parse_result.is_ok(), true);

        let norwegian_id = norwegian_id_parse_result.unwrap();

        assert_eq!(norwegian_id.get_id_type(), IdType::BirthNumber);
        assert_eq!(norwegian_id.is_female(), true);
    }
}

#[test]
fn test_invalid_ids_with_invalid_length() {
    for number in NUMBERS_WITH_INVALID_LENGTH {
        let norwegian_id_parse_result = NorwegianId::parse(number);
        assert_eq!(norwegian_id_parse_result.is_err(), true);

        let error = norwegian_id_parse_result.unwrap_err();
        assert_eq!(error, NorwegianIdValidationError::InvalidLength);
    }
}

#[test]
fn test_invalid_ids_with_invalid_checksum() {
    for number in NUMBERS_WITH_INVALID_CHECKSUM {
        let norwegian_id_parse_result = NorwegianId::parse(number);
        assert_eq!(norwegian_id_parse_result.is_err(), true);

        let error = norwegian_id_parse_result.unwrap_err();
        assert_eq!(error, NorwegianIdValidationError::InvalidChecksum);
    }
}

#[test]
fn test_valid_birth_numbers_from_norsk_helsenett_testaktoerer() {
    for number in VALID_BIRTH_NUMBERS_NORSK_HELSENETT_TESTAKTOERER {
        let norwegian_id_parse_result = NorwegianId::parse(number);
        assert_eq!(norwegian_id_parse_result.is_ok(), true);

        let norwegian_id = norwegian_id_parse_result.unwrap();
        assert_eq!(norwegian_id.get_id_type(), IdType::BirthNumber);
    }
}

#[test]
fn test_valid_fh_numbers_from_norsk_helsenett_testaktoerer() {
    for number in VALID_FHNUMBER_FROM_NORSK_HELSENETT_TESTAKTOERER {
        let norwegian_id_parse_result = NorwegianId::parse(number);
        assert_eq!(norwegian_id_parse_result.is_ok(), true);

        let norwegian_id = norwegian_id_parse_result.unwrap();
        assert_eq!(norwegian_id.get_id_type(), IdType::FHNumber);
    }
}

#[test]
fn test_valid_d_numbers_from_norsk_helsenett_testaktoerer() {
    for number in VALID_DNUMBER_FROM_NORSK_HELSENETT_TESTAKTOERER {
        let norwegian_id_parse_result = NorwegianId::parse(number);
        assert_eq!(norwegian_id_parse_result.is_ok(), true);

        let norwegian_id = norwegian_id_parse_result.unwrap();
        assert_eq!(norwegian_id.get_id_type(), IdType::DNumber);
    }
}

#[test]
fn test_valid_h_numbers_from_norsk_helsenett_testaktoerer() {
    for number in VALID_HNUMBER_FROM_NORSK_HELSENETT_TESTAKTOERER {
        let norwegian_id_parse_result = NorwegianId::parse(number);
        assert_eq!(norwegian_id_parse_result.is_ok(), true);

        let norwegian_id = norwegian_id_parse_result.unwrap();
        assert_eq!(norwegian_id.get_id_type(), IdType::HNumber);
    }
}

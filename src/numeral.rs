pub const MAX_NUMERAL_VALUE: u16 = 3_999;

const NUMERALS: &[(u16, &str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub fn to_numeral(value: u16) -> Option<String> {
    if value > MAX_NUMERAL_VALUE {
        return None;
    }

    let mut s = String::new();
    let mut value = value;

    for &(number, numeral) in NUMERALS {
        while value >= number {
            value = value - number;
            s.push_str(numeral)
        }
    }

    Some(s)
}

#[cfg(test)]
mod tests {
    use super::to_numeral;

    #[test]
    fn test_values() {
        let test_cases = &[
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
            (3, "III"),
            (14, "XIV"),
            (58, "LVIII"),
            (1994, "MCMXCIV"),
            (2023, "MMXXIII"),
            (2024, "MMXXIV"),
            (3999, "MMMCMXCIX"),
        ];

        for &(value, expected) in test_cases {
            assert_eq!(
                to_numeral(value),
                Some(expected.to_string()),
                "{} did not equal {}",
                value,
                expected
            );
        }
    }

    #[test]
    fn test_invalid_values() {
        assert!(to_numeral(4000).is_none());
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::datetime_move::movement::*;

    #[test]
    fn test_date_time_move_support_parse_simple_set() {
        let input = String::from("Y2024");
        let movements = DateTimeMoveSupport::parse(&input).unwrap();
        assert_eq!(movements.len(), 1);
        assert!(matches!(movements[0].unit, DateTimeMovementUnit::Year));
        assert!(matches!(movements[0].r#type, DateTimeMovementType::Set));
        assert_eq!(movements[0].offset, 2024);
    }

    #[test]
    fn test_date_time_move_support_parse_plus() {
        let input = String::from("M+5");
        let movements = DateTimeMoveSupport::parse(&input).unwrap();
        assert_eq!(movements.len(), 1);
        assert!(matches!(movements[0].unit, DateTimeMovementUnit::Month));
        assert!(matches!(movements[0].r#type, DateTimeMovementType::Plus));
        assert_eq!(movements[0].offset, 5);
    }

    #[test]
    fn test_date_time_move_support_parse_minus() {
        let input = String::from("D-10");
        let movements = DateTimeMoveSupport::parse(&input).unwrap();
        assert_eq!(movements.len(), 1);
        assert!(matches!(movements[0].unit, DateTimeMovementUnit::Day));
        assert!(matches!(movements[0].r#type, DateTimeMovementType::Minus));
        assert_eq!(movements[0].offset, 10);
    }

    #[test]
    fn test_date_time_move_support_parse_multiple_moves() {
        let input = String::from("Y2024 M+5 D-10");
        let movements = DateTimeMoveSupport::parse(&input).unwrap();
        assert_eq!(movements.len(), 3);
        assert!(matches!(movements[0].unit, DateTimeMovementUnit::Year));
        assert!(matches!(movements[1].unit, DateTimeMovementUnit::Month));
        assert!(matches!(movements[2].unit, DateTimeMovementUnit::Day));
        assert!(matches!(movements[0].r#type, DateTimeMovementType::Set));
        assert!(matches!(movements[1].r#type, DateTimeMovementType::Plus));
        assert!(matches!(movements[2].r#type, DateTimeMovementType::Minus));
    }

    #[test]
    fn test_date_time_move_support_parse_with_whitespace() {
        let input = String::from("  h + 30   m - 15  ");
        let movements = DateTimeMoveSupport::parse(&input).unwrap();
        assert_eq!(movements.len(), 2);
        assert!(matches!(movements[0].unit, DateTimeMovementUnit::Hour));
        assert!(matches!(movements[1].unit, DateTimeMovementUnit::Minute));
        assert!(matches!(movements[0].r#type, DateTimeMovementType::Plus));
        assert!(matches!(movements[1].r#type, DateTimeMovementType::Minus));
        assert_eq!(movements[0].offset, 30);
        assert_eq!(movements[1].offset, 15);
    }

    #[test]
    fn test_date_time_move_support_parse_time_units() {
        let input = String::from("h12 m30 s45");
        let movements = DateTimeMoveSupport::parse(&input).unwrap();
        assert_eq!(movements.len(), 3);
        assert!(matches!(movements[0].unit, DateTimeMovementUnit::Hour));
        assert!(matches!(movements[1].unit, DateTimeMovementUnit::Minute));
        assert!(matches!(movements[2].unit, DateTimeMovementUnit::Second));
        assert!(matches!(movements[0].r#type, DateTimeMovementType::Set));
        assert!(matches!(movements[1].r#type, DateTimeMovementType::Set));
        assert!(matches!(movements[2].r#type, DateTimeMovementType::Set));
        assert_eq!(movements[0].offset, 12);
        assert_eq!(movements[1].offset, 30);
        assert_eq!(movements[2].offset, 45);
    }

    #[test]
    fn test_date_time_move_support_parse_empty_string() {
        let input = String::from("");
        let movements = DateTimeMoveSupport::parse(&input).unwrap();
        assert_eq!(movements.len(), 0);
    }

    #[test]
    fn test_date_time_move_support_parse_whitespace_only() {
        let input = String::from("   ");
        let movements = DateTimeMoveSupport::parse(&input).unwrap();
        assert_eq!(movements.len(), 0);
    }

    #[test]
    fn test_date_time_move_support_parse_invalid_unit() {
        let input = String::from("X123");
        let result = DateTimeMoveSupport::parse(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_date_time_move_support_parse_no_offset() {
        let input = String::from("Y");
        let result = DateTimeMoveSupport::parse(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_date_time_move_support_parse_digit_without_unit() {
        let input = String::from("123");
        let result = DateTimeMoveSupport::parse(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_date_time_move_support_parse_multiple_types_without_offset() {
        let input = String::from("Y+-10");
        let result = DateTimeMoveSupport::parse(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_date_time_move_support_parse_invalid_character() {
        let input = String::from("Y@123");
        let result = DateTimeMoveSupport::parse(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_date_time_move_support_parse_incomplete_move() {
        let input = String::from("Y123 M");
        let result = DateTimeMoveSupport::parse(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_date_time_move_support_parse_digit_after_move() {
        let input = String::from("Y2024 123");
        let result = DateTimeMoveSupport::parse(&input);
        assert!(result.is_err());
    }
}

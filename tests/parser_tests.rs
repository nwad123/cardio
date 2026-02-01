use cardio::parser::*;
use cardio::property::*;

mod lex_tests {
    use super::*;

    #[test]
    fn comments() {
        let input = "// This is a comment";
        let tokens = lex(input);
        assert!(tokens.is_empty());
        
        let input = "// Multiple comments with empty lines\n\n// Comment again!\n   // Comment with leading spaces\n";
        let tokens = lex(input);
        assert!(tokens.is_empty());
    }

    #[test]
    #[should_panic]
    fn random_letters() {
        let input = "//Comment followed by letters\nabc";
        let tokens = lex(input);
    }

}

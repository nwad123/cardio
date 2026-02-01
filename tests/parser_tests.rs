use cardio::parser::*;
use cardio::property::*;

#[cfg(test)]
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

    #[test]
    fn boolean_lexing() {
        assert_eq!(lex("true"), vec![Token::True]);
        assert_eq!(lex("false"), vec![Token::False]);
    }

    #[test]
    fn numeric_lexing() {
        assert_eq!(lex("42"), vec![Token::Integer(42)]);
        assert_eq!(lex("3.14"), vec![Token::Float(3.14)]);
        assert_eq!(lex(".5"), vec![Token::Float(0.5)]);
        assert_eq!(lex("10."), vec![Token::Float(10.0)]);
    }

    #[test]
    fn operator_lexing() {
        assert_eq!(lex("+ - *"), vec![Token::Plus, Token::Minus, Token::Multiply]);
    }

    #[test]
    fn comparison_lexing() {
        assert_eq!(lex("== !="), vec![Token::Equal, Token::NotEqual]);
        assert_eq!(lex("< >"), vec![Token::LessThan, Token::GreaterThan]);
        assert_eq!(lex("<= >="), vec![Token::LessThanOrEqual, Token::GreaterThanOrEqual]);
        assert_eq!(lex("=?"), vec![Token::ValueQuery]);
    }

    #[test]
    fn string_lexing() {
         assert_eq!(lex("\"hello world\""), vec![Token::StringLabel("\"hello world\"".to_string())]);
         assert_eq!(lex(":"), vec![Token::Colon]);
    }

    #[test]
    fn structural_lexing() {
         assert_eq!(lex("! & |"), vec![Token::Not, Token::And, Token::Or]);
         assert_eq!(lex("( ) [ ] { }"), vec![
             Token::LParen, Token::RParen, 
             Token::LBracket, Token::RBracket, 
             Token::LBrace, Token::RBrace
         ]);
         assert_eq!(lex(","), vec![Token::Comma]);
    }

    #[test]
    fn keyword_lexing() {
        assert_eq!(lex("P S"), vec![Token::ProbabilityQuery, Token::SteadyStateQuery]);
        assert_eq!(lex("G X U"), vec![Token::Globally, Token::Next, Token::Until]);
    }

}

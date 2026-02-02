use crate::parser::{lex, Token};

#[cfg(test)]
mod tests {
    use super::{lex, Token};

    macro_rules! lex_test {
        ($($name:ident: $input:expr, $expected:expr,)*) => {
            $( 
                #[test]
                fn $name() {
                    assert_eq!(lex($input), $expected);
                }
            )*
        }
    }

    lex_test! {
        comments_empty: "// This is a comment", vec![],
        comments_multi: "// Multiple comments with empty lines\n\n// Comment again!\n   // Comment with leading spaces\n", vec![],

        boolean_true: "true", vec![Token::True],
        boolean_false: "false", vec![Token::False],

        numeric_integer: "42", vec![Token::Integer(42)],
        numeric_float: "3.14", vec![Token::Float(3.14)],
        numeric_float_point: ".5", vec![Token::Float(0.5)],
        numeric_float_ten: "10.", vec![Token::Float(10.0)],

        operator_arithmetic: "+ - *", vec![Token::Plus, Token::Minus, Token::Multiply],

        comparison_equal: "== !=", vec![Token::Equal, Token::NotEqual],
        comparison_rel: "< >", vec![Token::LessThan, Token::GreaterThan],
        comparison_rel_eq: "<= >=", vec![Token::LessThanOrEqual, Token::GreaterThanOrEqual],
        comparison_query: "=?", vec![Token::ValueQuery],

        string_hello: "\"hello world\"", vec![Token::StringLabel("\"hello world\"".to_string())],
        token_colon: ":", vec![Token::Colon],

        structural_ops: "! & |", vec![Token::Not, Token::And, Token::Or],
        structural_brackets: "( ) [ ] { }", vec![
            Token::LParen, Token::RParen, 
            Token::LBracket, Token::RBracket, 
            Token::LBrace, Token::RBrace
        ],
        structural_comma: ",", vec![Token::Comma],

        keyword_prob: "P S", vec![Token::ProbabilityQuery, Token::SteadyStateQuery],
        keyword_path: "G X U", vec![Token::Globally, Token::Next, Token::Until],
    }

    #[test]
    #[should_panic]
    fn random_letters() {
        let input = "//Comment followed by letters\nabc";
        let _tokens = lex(input);
    }
}

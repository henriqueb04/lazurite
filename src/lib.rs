mod lexer;

pub use crate::lexer::{Lexer, TokenType};

// pub enum Result {
//     Integer(isize),
//     None
// }

// pub fn eval_expr(_expr: &str) -> Result {
//     Result::None
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = eval_expr("1 + 2");
    //     let Result::Integer(i) = result else {
    //         panic!("Result not an Integer!");
    //     };
    //     assert_eq!(i, 3)
    // }

    #[test]
    fn tokenize() {
        let mut lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number(5));

        let mut lexer = Lexer::new("bazinga");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Identifier(String::from("bazinga")));

        let mut lexer = Lexer::new("9");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_ne!(tokens[0].token_type, TokenType::Number(5));

        let mut lexer = Lexer::new("9 + 5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Number(9));
        assert_eq!(tokens[1].token_type, TokenType::Plus);
        assert_eq!(tokens[2].token_type, TokenType::Number(5));

        let mut lexer = Lexer::new("3d6c5kh1");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Number(3));
        assert_eq!(tokens[1].token_type, TokenType::Dice);
        assert_eq!(tokens[2].token_type, TokenType::Number(6));
        assert_eq!(tokens[3].token_type, TokenType::DiceOption(String::from("c5kh1")));

        let mut lexer = Lexer::new("d20");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Dice);
        assert_eq!(tokens[1].token_type, TokenType::Number(20));

        let mut lexer = Lexer::new("(a + 1)d(2 * variavel)c5r");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 12);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::Identifier(String::from("a")));
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[3].token_type, TokenType::Number(1));
        assert_eq!(tokens[4].token_type, TokenType::RightParen);
        assert_eq!(tokens[5].token_type, TokenType::Dice);
        assert_eq!(tokens[6].token_type, TokenType::LeftParen);
        assert_eq!(tokens[7].token_type, TokenType::Number(2));
        assert_eq!(tokens[8].token_type, TokenType::Asterisk);
        assert_eq!(tokens[9].token_type, TokenType::Identifier(String::from("variavel")));
        assert_eq!(tokens[10].token_type, TokenType::RightParen);
        assert_eq!(tokens[11].token_type, TokenType::DiceOption(String::from("c5r")));
    }
}

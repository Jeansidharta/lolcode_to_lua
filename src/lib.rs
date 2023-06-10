use lolcode_ast::lexer::{NumberToken, Token, TokenType, TokenValue};
use lolcode_ast::parser::expression::{ASTExpression, ASTType, Identifier, VariableAccess};
use lolcode_ast::parser::statements::assignment::VariableAssignment;
use lolcode_ast::parser::statements::bukkit_set_slot::BukkitSetSlot;
use lolcode_ast::parser::statements::how_is_i::HowIzI;
use lolcode_ast::parser::statements::i_has_a::{IHasA, IHasAInitialValue};
use lolcode_ast::parser::statements::i_is::IIz;
use lolcode_ast::parser::statements::im_in_yr::{ImInYr, LoopCondition, LoopOperation};
use lolcode_ast::parser::statements::visible::Visible;
use lolcode_ast::parser::statements::ASTNode;
use lolcode_ast::parser::ASTBlock;
use lolcode_ast::tokenize_and_parse;
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn make_ast_from_file(filename: PathBuf) -> Result<ASTBlock, String> {
    let file_contents = read_to_string(&filename)
        .map_err(|_| format!("Could not read file {}", filename.to_str().unwrap()))?;

    tokenize_and_parse(file_contents).map_err(|_| "Failed to tokenize file".to_string())
}

pub trait ToLua {
    fn into_lua(self) -> String;
}

impl ToLua for ASTNode {
    fn into_lua(self) -> String {
        match self {
            ASTNode::HAI(_) => "".to_string(),
            ASTNode::IHasA(IHasA {
                identifier,
                initial_value,
            }) => match initial_value {
                Some(value) => format!("local {} = {}", identifier.into_lua(), value.into_lua()),
                None => format!("local {}", identifier.into_lua()),
            },
            ASTNode::ImInYr(ImInYr {
                on_iteration,
                condition,
                code_block,
                ..
            }) => {
                let condition = condition.map(|condition| match condition {
                    LoopCondition::TIL(expression) => format!("!{}", expression.into_lua()),
                    LoopCondition::WILE(expression) => format!("{}", expression.into_lua()),
                });
                let on_iteration = on_iteration.map(|iter| match iter.operation {
                    LoopOperation::UPPIN(_) => format!("{} += 1", iter.operand.into_lua()),
                    LoopOperation::NERFIN(_) => format!("{} -= 1", iter.operand.into_lua()),
                });

                let code_block = code_block.into_lua();

                format!(
                    "while {} do\n{}\n{}\nend",
                    condition.unwrap_or("".to_string()),
                    code_block,
                    on_iteration.unwrap_or("".to_string())
                )
            }
            ASTNode::BukkitSetSlot(BukkitSetSlot {
                bukkit,
                slot_name,
                value,
            }) => format!(
                "{}[{}] = {}",
                bukkit.into_lua(),
                slot_name.into_lua(),
                value.into_lua()
            ),
            ASTNode::VariableAssignment(VariableAssignment { identifier, value }) => {
                format!("{} = {}", identifier.into_lua(), value.into_lua())
            }
            ASTNode::Visible(Visible(expressions, no_new_line)) => format!(
                "io.write({}{})",
                expressions
                    .into_iter()
                    .map(|i| i.into_lua())
                    .fold("".to_string(), |acc, item| format!("{}, {}", acc, item)),
                if no_new_line.is_some() { "" } else { "\n" }
            ),
            ASTNode::FoundYr(expression) => format!("return {}", expression.into_lua()),
            ASTNode::Wtf(_) => todo!(),
            ASTNode::ORly(_) => todo!(),
            ASTNode::IIz(IIz { name, arguments }) => format!(
                "{}({})",
                name.into_lua(),
                arguments
                    .into_iter()
                    .map(|a| a.into_lua())
                    .fold("".to_string(), |acc, item| format!("{}, {}", acc, item))
            ),
            ASTNode::HowIzI(HowIzI {
                name,
                arguments,
                body,
            }) => {
                format!(
                    "function {} ({})\n{}\nend",
                    name.into_lua(),
                    arguments
                        .into_iter()
                        .map(|a| a.into_lua())
                        .fold("".to_string(), |acc, item| format!("{}, {}", acc, item)),
                    body.into_lua()
                )
            }
            ASTNode::Gtfo(_) => format!("return nil"),
            ASTNode::Gimmeh(_) => todo!(),
            ASTNode::Expression(expression) => format!("it = {}", expression.into_lua()),
            ASTNode::ASTError(_) => todo!(),
            ASTNode::KTHXBYE(_) => format!(""),
        }
    }
}

impl ToLua for ASTBlock {
    fn into_lua(self) -> String {
        self.0
            .into_iter()
            .map(|statement| statement.into_lua())
            .fold(String::new(), |acc, s| format!("{}\n{}", acc, s))
    }
}

impl ToLua for ASTExpression {
    fn into_lua(self) -> String {
        match self {
            ASTExpression::Not(expr) => format!("(not {})", expr.into_lua()),
            ASTExpression::Maek(expr, _) => expr.into_lua(),
            ASTExpression::WonOf(left, right) => {
                format!("({} ~= {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::AllOf(expressions) => format!(
                "({})",
                expressions
                    .into_iter()
                    .enumerate()
                    .map(|(index, expr)| {
                        let expr = expr.into_lua();
                        if index == 0 {
                            format!("{}", expr)
                        } else {
                            format!("and {}", expr)
                        }
                    })
                    .collect::<String>()
            ),
            ASTExpression::AnyOf(expressions) => format!(
                "({})",
                expressions
                    .into_iter()
                    .enumerate()
                    .map(|(index, expr)| {
                        let expr = expr.into_lua();
                        if index == 0 {
                            format!("{}", expr)
                        } else {
                            format!("or {}", expr)
                        }
                    })
                    .collect::<String>()
            ),
            ASTExpression::Smoosh(expressions) => format!(
                "({})",
                expressions
                    .into_iter()
                    .enumerate()
                    .map(|(index, expr)| {
                        let expr = expr.into_lua();
                        if index == 0 {
                            format!("{}", expr)
                        } else {
                            format!(".. {}", expr)
                        }
                    })
                    .collect::<String>()
            ),
            ASTExpression::ModOf(left, right) => {
                format!("math.fmod({}, {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::BothOf(left, right) => {
                format!("({} and {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::SumOf(left, right) => {
                format!("({} + {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::DiffOf(left, right) => {
                format!("({} - {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::VariableAccess(access) => access.into_lua(),
            ASTExpression::LiteralValue(value) => value.into_lua(),
            ASTExpression::BiggrOf(left, right) => {
                format!("({} > {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::SmallrOf(left, right) => {
                format!("({} < {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::EitherOf(left, right) => {
                format!("({} or {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::ProduktOf(left, right) => {
                format!("({} * {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::QuoshuntOf(left, right) => {
                format!("({} / {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::BothSaem(left, right) => {
                format!("({} == {})", left.into_lua(), right.into_lua())
            }
            ASTExpression::Diffrint(left, right) => {
                format!("({} ~= {})", left.into_lua(), right.into_lua())
            }
        }
    }
}

impl ToLua for IHasAInitialValue {
    fn into_lua(self) -> String {
        match self {
            IHasAInitialValue::Type(ast_type) => match ast_type {
                ASTType::Troof => false.to_string(),
                ASTType::Numbr => 0.to_string(),
                ASTType::Noob => "NIL".to_string(),
                ASTType::Numbar => "0.0".to_string(),
                ASTType::Bukkit => "{}".to_string(),
                ASTType::Yarn => "\"\"".to_string(),
            },
            IHasAInitialValue::Expression(expression) => expression.into_lua(),
        }
    }
}

impl ToLua for Token {
    fn into_lua(self) -> String {
        match self.token_type {
            TokenType::Comma => ",".to_string(),
            TokenType::Ellipsis => "...".to_string(),
            TokenType::QuestionMark => "?".to_string(),
            TokenType::ExclamationMark => "!".to_string(),
            TokenType::CommentMultiLine(_) => "".to_string(),
            TokenType::CommentSingleLine(_) => "".to_string(),
            TokenType::Keyword(keyword) => keyword.into_str().to_string(),
            TokenType::Identifier(identifier) => identifier,
            TokenType::Value(value) => match value {
                TokenValue::NOOB => "NIL".to_string(),
                TokenValue::String(str) => str,
                TokenValue::Number(NumberToken::Int(int)) => int.to_string(),
                TokenValue::Number(NumberToken::Float(float)) => float.to_string(),
                TokenValue::Boolean(bool) => bool.to_string(),
            },
            TokenType::Symbol(symbol) => symbol,
            TokenType::BukkitSlotAccess => todo!(),
        }
    }
}

impl ToLua for Identifier {
    fn into_lua(self) -> String {
        let Self { name, is_srs } = self;
        if is_srs {
            panic!("Cannot convert SRS tokens to lua");
        }
        format!("{}", name.into_lua())
    }
}

impl ToLua for VariableAccess {
    fn into_lua(self) -> String {
        let Self { name, accesses } = self;

        format!("{}", name.into_lua())
    }
}

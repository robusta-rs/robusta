use robusta_lexer::TokenKind;
use robusta_syntax::operator::{AssignOp, BinaryOp, UpdateOp};

pub fn update_op(kind: TokenKind) -> Option<UpdateOp> {
    Some(match kind {
        TokenKind::PlusPlus => UpdateOp::Increment,
        TokenKind::MinusMinus => UpdateOp::Decrement,
        _ => return None,
    })
}

pub fn assign_op(kind: TokenKind) -> Option<AssignOp> {
    Some(match kind {
        TokenKind::Assign => AssignOp::Assign,
        TokenKind::PlusEq => AssignOp::Add,
        TokenKind::MinusEq => AssignOp::Sub,
        TokenKind::StarEq => AssignOp::Mul,
        TokenKind::SlashEq => AssignOp::Div,
        TokenKind::PercentEq => AssignOp::Rem,
        TokenKind::AmpEq => AssignOp::BitAnd,
        TokenKind::PipeEq => AssignOp::BitOr,
        TokenKind::CaretEq => AssignOp::BitXor,
        TokenKind::LtLtEq => AssignOp::Shl,
        _ => return None,
    })
}

pub fn binary_op(kind: TokenKind) -> Option<BinaryOp> {
    Some(match kind {
        TokenKind::PipePipe => BinaryOp::Or,
        TokenKind::AmpAmp => BinaryOp::And,
        TokenKind::Pipe => BinaryOp::BitOr,
        TokenKind::Caret => BinaryOp::BitXor,
        TokenKind::Amp => BinaryOp::BitAnd,
        TokenKind::EqEq => BinaryOp::Eq,
        TokenKind::BangEq => BinaryOp::NotEq,
        TokenKind::Lt => BinaryOp::Lt,
        TokenKind::Gt => BinaryOp::Gt,
        TokenKind::LtEq => BinaryOp::LtEq,
        TokenKind::GtEq => BinaryOp::GtEq,
        TokenKind::LtLt => BinaryOp::Shl,
        TokenKind::Plus => BinaryOp::Add,
        TokenKind::Minus => BinaryOp::Sub,
        TokenKind::Star => BinaryOp::Mul,
        TokenKind::Slash => BinaryOp::Div,
        TokenKind::Percent => BinaryOp::Rem,
        _ => return None,
    })
}

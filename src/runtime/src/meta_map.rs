use crate::ValueString;

use {
    crate::Value,
    indexmap::IndexMap,
    koto_parser::MetaKeyId,
    rustc_hash::FxHasher,
    std::{
        fmt,
        hash::{BuildHasherDefault, Hash},
    },
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    Index,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BinaryOp::*;

        write!(
            f,
            "{}",
            match self {
                Add => "+",
                Subtract => "-",
                Multiply => "*",
                Divide => "/",
                Modulo => "%",
                Less => "<",
                LessOrEqual => "<=",
                Greater => ">",
                GreaterOrEqual => ">=",
                Equal => "==",
                NotEqual => "!=",
                Index => "[]",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnaryOp {
    Negate,
    Display,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use UnaryOp::*;

        write!(
            f,
            "{}",
            match self {
                Negate => "negate",
                Display => "display",
            }
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum MetaKey {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Test(ValueString),
    Tests,
    PreTest,
    PostTest,
    Type,
}

impl From<BinaryOp> for MetaKey {
    fn from(op: BinaryOp) -> Self {
        Self::BinaryOp(op)
    }
}

impl From<UnaryOp> for MetaKey {
    fn from(op: UnaryOp) -> Self {
        Self::UnaryOp(op)
    }
}

pub fn meta_id_to_key(id: MetaKeyId, name: Option<&str>) -> Result<MetaKey, String> {
    use {BinaryOp::*, UnaryOp::*};

    let result = match id {
        MetaKeyId::Add => MetaKey::BinaryOp(Add),
        MetaKeyId::Subtract => MetaKey::BinaryOp(Subtract),
        MetaKeyId::Multiply => MetaKey::BinaryOp(Multiply),
        MetaKeyId::Divide => MetaKey::BinaryOp(Divide),
        MetaKeyId::Modulo => MetaKey::BinaryOp(Modulo),
        MetaKeyId::Less => MetaKey::BinaryOp(Less),
        MetaKeyId::LessOrEqual => MetaKey::BinaryOp(LessOrEqual),
        MetaKeyId::Greater => MetaKey::BinaryOp(Greater),
        MetaKeyId::GreaterOrEqual => MetaKey::BinaryOp(GreaterOrEqual),
        MetaKeyId::Equal => MetaKey::BinaryOp(Equal),
        MetaKeyId::NotEqual => MetaKey::BinaryOp(NotEqual),
        MetaKeyId::Index => MetaKey::BinaryOp(Index),
        MetaKeyId::Negate => MetaKey::UnaryOp(Negate),
        MetaKeyId::Display => MetaKey::UnaryOp(Display),
        MetaKeyId::Tests => MetaKey::Tests,
        MetaKeyId::Test => MetaKey::Test(
            name.ok_or_else(|| "Missing name for test".to_string())?
                .into(),
        ),
        MetaKeyId::PreTest => MetaKey::PreTest,
        MetaKeyId::PostTest => MetaKey::PostTest,
        MetaKeyId::Type => MetaKey::Type,
        MetaKeyId::Invalid => return Err("Invalid MetaKeyId".to_string()),
    };

    Ok(result)
}

pub type MetaMap = IndexMap<MetaKey, Value, BuildHasherDefault<FxHasher>>;

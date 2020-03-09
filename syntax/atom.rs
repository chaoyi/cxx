use crate::syntax::Type;
use proc_macro2::Ident;

#[derive(Copy, Clone, PartialEq)]
pub enum Atom {
    Bool,
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
    CxxString,
    RustString,
}

impl Atom {
    pub fn from(ident: &Ident) -> Option<Self> {
        use self::Atom::*;
        match ident.to_string().as_str() {
            "bool" => Some(Bool),
            "u8" => Some(U8),
            "u16" => Some(U16),
            "u32" => Some(U32),
            "u64" => Some(U64),
            "usize" => Some(Usize),
            "i8" => Some(I8),
            "i16" => Some(I16),
            "i32" => Some(I32),
            "i64" => Some(I64),
            "isize" => Some(Isize),
            "f32" => Some(F32),
            "f64" => Some(F64),
            "CxxString" => Some(CxxString),
            "String" => Some(RustString),
            _ => None,
        }
    }

    pub fn to_cxx(&self) -> &'static str {
        use self::Atom::*;
        match self {
            Bool => "bool",
            U8 => "uint8_t",
            U16 => "uint16_t",
            U32 => "uint32_t",
            U64 => "uint64_t",
            Usize => "size_t",
            I8 => "int8_t",
            I16 => "int16_t",
            I32 => "int32_t",
            I64 => "int64_t",
            Isize => "ssize_t",
            F32 => "float",
            F64 => "double",
            CxxString => "::std::string",
            RustString => "::rust::String",
        }
    }
}

impl PartialEq<Atom> for Ident {
    fn eq(&self, atom: &Atom) -> bool {
        Atom::from(self) == Some(*atom)
    }
}

impl PartialEq<Atom> for Type {
    fn eq(&self, atom: &Atom) -> bool {
        match self {
            Type::Ident(ident) => ident == atom,
            _ => false,
        }
    }
}

impl PartialEq<Atom> for &Ident {
    fn eq(&self, atom: &Atom) -> bool {
        *self == atom
    }
}

impl PartialEq<Atom> for &Type {
    fn eq(&self, atom: &Atom) -> bool {
        *self == atom
    }
}

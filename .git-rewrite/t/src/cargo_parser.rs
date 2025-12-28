// 🔥 Estructuras para parsear JSON de cargo check

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerMessage {
    pub reason: String,
    pub message: Message,
    pub package_id: Option<String>,
    pub target: Option<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
    pub code: Option<Code>,
    pub level: String,
    pub spans: Vec<Span>,
    pub children: Vec<Message>,
    pub rendered: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Code {
    pub code: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub file_name: String,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub is_primary: bool,
    pub text: Vec<SpanText>,
    pub label: Option<String>,
    pub suggested_replacement: Option<String>,
    pub suggestion_applicability: Option<String>,
    pub expansion: Option<Box<Expansion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanText {
    pub text: String,
    pub highlight_start: usize,
    pub highlight_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expansion {
    pub span: Span,
    pub macro_decl_name: String,
    pub def_site_span: Option<Span>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub kind: Vec<String>,
    pub crate_types: Vec<String>,
    pub name: String,
    pub src_path: String,
    pub edition: String,
    pub doc: bool,
    pub doctest: bool,
    pub test: bool,
}

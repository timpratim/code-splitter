use pyo3::prelude::*;
use tree_sitter::Language as TreeSitterLanguage;

#[pyclass(eq)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum Language {
    Golang,
    Markdown,
    Python,
    Rust,
    JavaScript,
    TypeScript,
    Java,
    Cpp,
    C,
    Ruby,
    CSharp,
    Swift,

}

impl Language {
    pub fn as_tree_sitter_language(&self) -> TreeSitterLanguage {
        match self {
            Language::Golang => tree_sitter_go::LANGUAGE.into(),
            Language::Markdown => tree_sitter_md::language(),
            Language::Python => tree_sitter_python::LANGUAGE.into(),
            Language::Rust => tree_sitter_rust::LANGUAGE.into(),
            Language::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
            Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Language::Java => tree_sitter_java::LANGUAGE.into(),
            Language::Cpp => tree_sitter_cpp::LANGUAGE.into(),
            Language::C => tree_sitter_c::LANGUAGE.into(),
            Language::Ruby => tree_sitter_ruby::LANGUAGE.into(),
            Language::CSharp => tree_sitter_c_sharp::LANGUAGE.into(),
            Language::Swift => tree_sitter_swift::LANGUAGE.into(),

        }
    }
}

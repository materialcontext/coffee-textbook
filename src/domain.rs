//! Domain model: what a book is, independent of how it is parsed or rendered.

use serde::Deserialize;

/// Book-level metadata, read from `book.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct BookMeta {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: String,
    pub edition: String,
    pub year: u16,
    pub publisher: Option<String>,
    pub license_holder: String,
    #[serde(default = "default_lang")]
    pub lang: String,
    /// Release version shown in every footer, e.g. "2.1".
    #[serde(default)]
    pub version: Option<String>,
    /// Name printed in each sold copy; set from the command line, not the file.
    #[serde(skip)]
    pub buyer: Option<String>,
}

fn default_lang() -> String {
    "en".into()
}

/// Chapter front matter.
#[derive(Debug, Clone, Deserialize)]
pub struct ChapterMeta {
    pub number: u8,
    /// Display label when it differs from the number ("2A", "B").
    #[serde(default)]
    pub label: Option<String>,
    pub title: String,
    /// One-line statement of what the chapter answers.
    pub question: String,
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default)]
    pub prerequisites: Vec<String>,
}

impl ChapterMeta {
    pub fn label(&self) -> String {
        self.label.clone().unwrap_or_else(|| self.number.to_string())
    }
}

/// Which part of the book a chapter belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Front,
    Main,
    Appendix,
}

#[derive(Debug, Clone)]
pub struct Chapter {
    pub slug: String,
    pub part: Part,
    pub meta: ChapterMeta,
    pub blocks: Vec<Block>,
}

/// A chapter is an ordered sequence of blocks.
#[derive(Debug, Clone)]
pub enum Block {
    Prose(String),
    Quiz(Quiz),
    Widget(WidgetSpec),
    /// Worked example: title and Markdown body.
    Example { title: String, body: String },
    /// "Status of the evidence" box: Markdown body.
    Evidence(String),
    Problems(Problems),
    Lab(Lab),
    Figure(Figure),
    Equations(Equations),
    /// Numbered display equation with its symbol list.
    Eq(DisplayEq),
    /// "Common misconception" box: Markdown body.
    Misconception(String),
    /// Optional "Going further" box: title and Markdown body.
    Deeper { title: String, body: String },
    /// Where the generated key-terms list goes.
    KeyTerms,
}

/// A display equation. `id` comes from the block header; `{eq:id}` refers to it.
#[derive(Debug, Clone, Deserialize)]
pub struct DisplayEq {
    #[serde(skip)]
    pub id: String,
    /// Inline HTML allowed (sub, sup, entities).
    pub formula: String,
    /// Rows of [symbol, meaning] or [symbol, meaning, unit].
    #[serde(default, rename = "where")]
    pub symbols: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Quiz {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(rename = "q")]
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Question {
    Choice { prompt: String, options: Vec<String>, answer: usize, explain: String },
    Numeric { prompt: String, answer: f64, tolerance: f64, #[serde(default)] unit: String, explain: String },
}

impl Question {
    pub fn prompt(&self) -> &str {
        match self {
            Question::Choice { prompt, .. } | Question::Numeric { prompt, .. } => prompt,
        }
    }
    pub fn explain(&self) -> &str {
        match self {
            Question::Choice { explain, .. } | Question::Numeric { explain, .. } => explain,
        }
    }
}

/// End-of-chapter problems. Solutions render on a separate page.
#[derive(Debug, Clone, Deserialize)]
pub struct Problems {
    #[serde(rename = "p")]
    pub items: Vec<Problem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Problem {
    /// "A" recall, "B" calculation, "C" analysis or design.
    pub tier: String,
    /// Section the problem practises, shown with the tier.
    #[serde(default)]
    pub section: Option<String>,
    pub prompt: String,
    pub solution: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Lab {
    pub id: String,
    pub title: String,
    pub aim: String,
    #[serde(default)]
    pub safety: Vec<String>,
    pub materials: Vec<String>,
    pub method: Vec<String>,
    /// Data-sheet column headings.
    pub columns: Vec<String>,
    /// Number of blank data rows.
    #[serde(default = "default_rows")]
    pub rows: usize,
    pub analysis: Vec<String>,
    #[serde(default)]
    pub expected: Option<String>,
}

fn default_rows() -> usize {
    6
}

#[derive(Debug, Clone, Deserialize)]
pub struct Figure {
    /// Optional id; `{fig:id}` in text refers to it.
    #[serde(default)]
    pub id: Option<String>,
    /// File name inside `content/figures`.
    pub file: String,
    pub alt: String,
    pub caption: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Equations {
    #[serde(rename = "eq")]
    pub items: Vec<Equation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Equation {
    pub name: String,
    /// Inline HTML allowed (sub, sup, entities).
    pub formula: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone)]
pub struct WidgetSpec {
    pub kind: String,
    pub params: toml::Table,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Term {
    pub term: String,
    pub definition: String,
}

#[derive(Debug, Clone)]
pub struct Book {
    pub meta: BookMeta,
    pub front: Vec<Chapter>,
    pub chapters: Vec<Chapter>,
    pub appendices: Vec<Chapter>,
    pub glossary: Vec<Term>,
}

impl Book {
    pub fn all(&self) -> impl Iterator<Item = &Chapter> {
        self.front.iter().chain(&self.chapters).chain(&self.appendices)
    }
}

//! Source text to domain values. Pure functions only; IO lives in `site`.
//!
//! Chapter format:
//!
//! ```text
//! +++
//! number = 2
//! title = "Extraction"
//! question = "How do solubles leave the grounds?"
//! +++
//! Markdown prose ...
//!
//! ~~~quiz
//! [[q]]
//! type = "choice"
//! ...
//! ~~~
//!
//! ~~~widget extraction
//! mode = "filter"
//! ~~~
//! ```
//!
//! Special blocks use `~~~` fences so that ordinary ``` code fences pass
//! through to the Markdown renderer untouched.

use crate::domain::{Block, Chapter, ChapterMeta, DisplayEq, Equations, Figure, Lab, Part, Problems, Quiz, WidgetSpec};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub line: usize,
    pub msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.msg)
    }
}

fn err(line: usize, msg: impl Into<String>) -> ParseError {
    ParseError { line, msg: msg.into() }
}

/// Split `+++` TOML front matter from the body. Returns (front matter, body, body start line).
pub fn split_front_matter(src: &str) -> Result<(&str, &str, usize), ParseError> {
    let rest = src
        .strip_prefix("+++\n")
        .ok_or_else(|| err(1, "missing +++ front matter"))?;
    let end = rest
        .find("\n+++\n")
        .ok_or_else(|| err(1, "unterminated front matter"))?;
    let fm = &rest[..end];
    let body = &rest[end + 5..];
    let body_line = fm.lines().count() + 3;
    Ok((fm, body, body_line))
}

/// Parse a whole chapter file.
pub fn parse_chapter(slug: &str, part: Part, src: &str) -> Result<Chapter, ParseError> {
    let src = src.replace("\r\n", "\n");
    let (fm, body, first) = split_front_matter(&src)?;
    let meta: ChapterMeta = toml::from_str(fm).map_err(|e| err(1, e.to_string()))?;
    let blocks = parse_blocks(body, first)?;
    Ok(Chapter { slug: slug.to_string(), part, meta, blocks })
}

/// Split a body into prose and special blocks.
pub fn parse_blocks(body: &str, first_line: usize) -> Result<Vec<Block>, ParseError> {
    let mut blocks = Vec::new();
    let mut prose = String::new();
    let mut lines = body.lines().enumerate().map(|(i, l)| (i + first_line, l));

    while let Some((n, line)) = lines.next() {
        let Some(header) = line.strip_prefix("~~~") else {
            prose.push_str(line);
            prose.push('\n');
            continue;
        };
        let header = header.trim();
        if header.is_empty() {
            return Err(err(n, "bare ~~~ fence; use ``` for code"));
        }
        let mut inner = String::new();
        let mut closed = false;
        for (_, l) in lines.by_ref() {
            if l.trim_end() == "~~~" {
                closed = true;
                break;
            }
            inner.push_str(l);
            inner.push('\n');
        }
        if !closed {
            return Err(err(n, format!("unterminated ~~~{header} block")));
        }
        flush_prose(&mut prose, &mut blocks);
        blocks.push(special_block(n, header, &inner)?);
    }
    flush_prose(&mut prose, &mut blocks);
    Ok(blocks)
}

fn flush_prose(prose: &mut String, blocks: &mut Vec<Block>) {
    if !prose.trim().is_empty() {
        blocks.push(Block::Prose(std::mem::take(prose)));
    } else {
        prose.clear();
    }
}

fn toml_block<T: serde::de::DeserializeOwned>(line: usize, inner: &str) -> Result<T, ParseError> {
    toml::from_str(inner).map_err(|e| err(line, e.to_string()))
}

fn special_block(line: usize, header: &str, inner: &str) -> Result<Block, ParseError> {
    let (kind, rest) = header.split_once(char::is_whitespace).unwrap_or((header, ""));
    let rest = rest.trim();
    match kind {
        "quiz" => {
            let q: Quiz = toml_block(line, inner)?;
            validate_quiz(line, &q)?;
            Ok(Block::Quiz(q))
        }
        "widget" => {
            if rest.is_empty() {
                return Err(err(line, "widget needs a kind"));
            }
            Ok(Block::Widget(WidgetSpec { kind: rest.to_string(), params: toml_block(line, inner)? }))
        }
        "example" => {
            if rest.is_empty() {
                return Err(err(line, "example needs a title"));
            }
            Ok(Block::Example { title: rest.to_string(), body: inner.to_string() })
        }
        "evidence" => Ok(Block::Evidence(inner.to_string())),
        "problems" => {
            let p: Problems = toml_block(line, inner)?;
            if let Some(bad) = p.items.iter().find(|x| !matches!(x.tier.as_str(), "A" | "B" | "C")) {
                return Err(err(line, format!("problem tier must be A, B or C, got `{}`", bad.tier)));
            }
            Ok(Block::Problems(p))
        }
        "lab" => Ok(Block::Lab(toml_block::<Lab>(line, inner)?)),
        "figure" => Ok(Block::Figure(toml_block::<Figure>(line, inner)?)),
        "equations" => Ok(Block::Equations(toml_block::<Equations>(line, inner)?)),
        "eq" => {
            if rest.is_empty() || rest.contains(char::is_whitespace) {
                return Err(err(line, "eq needs a single-word id"));
            }
            let mut e: DisplayEq = toml_block(line, inner)?;
            if e.symbols.iter().any(|r| r.len() < 2 || r.len() > 3) {
                return Err(err(line, "eq: each `where` row needs a symbol, a meaning and optionally a unit"));
            }
            e.id = rest.to_string();
            Ok(Block::Eq(e))
        }
        "misconception" => Ok(Block::Misconception(inner.to_string())),
        "deeper" => Ok(Block::Deeper { title: rest.to_string(), body: inner.to_string() }),
        "keyterms" => Ok(Block::KeyTerms),
        other => Err(err(line, format!("unknown block type `{other}`"))),
    }
}

fn validate_quiz(line: usize, q: &Quiz) -> Result<(), ParseError> {
    use crate::domain::Question::*;
    for (i, question) in q.questions.iter().enumerate() {
        match question {
            Choice { options, answer, .. } if *answer >= options.len() => {
                return Err(err(line, format!("question {}: answer index out of range", i + 1)))
            }
            Numeric { tolerance, .. } if *tolerance < 0.0 => {
                return Err(err(line, format!("question {}: negative tolerance", i + 1)))
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Question;

    const SAMPLE: &str = "+++\nnumber = 1\ntitle = \"T\"\nquestion = \"Q?\"\n+++\nHello.\n\n~~~quiz\n[[q]]\ntype = \"choice\"\nprompt = \"p\"\noptions = [\"a\", \"b\"]\nanswer = 1\nexplain = \"e\"\n~~~\n\nMore.\n\n~~~widget extraction\nmode = \"filter\"\n~~~\n";

    #[test]
    fn parses_blocks_in_order() {
        let c = parse_chapter("ch01", Part::Main, SAMPLE).unwrap();
        assert_eq!(c.meta.number, 1);
        assert_eq!(c.blocks.len(), 4);
        assert!(matches!(&c.blocks[0], Block::Prose(s) if s.contains("Hello")));
        assert!(matches!(&c.blocks[1], Block::Quiz(q) if matches!(q.questions[0], Question::Choice{answer: 1, ..})));
        assert!(matches!(&c.blocks[3], Block::Widget(w) if w.kind == "extraction"));
    }

    #[test]
    fn rejects_bad_answer_index() {
        let bad = SAMPLE.replace("answer = 1", "answer = 5");
        assert!(parse_chapter("x", Part::Main, &bad).is_err());
    }

    #[test]
    fn rejects_unterminated_block() {
        let bad = "+++\nnumber = 1\ntitle = \"T\"\nquestion = \"Q\"\n+++\n~~~quiz\n[[q]]\n";
        let e = parse_chapter("x", Part::Main, bad).unwrap_err();
        assert!(e.msg.contains("unterminated"));
    }

    #[test]
    fn code_fences_stay_prose() {
        let src = "+++\nnumber = 1\ntitle = \"T\"\nquestion = \"Q\"\n+++\n```\ncode\n```\n";
        let c = parse_chapter("x", Part::Main, src).unwrap();
        assert_eq!(c.blocks.len(), 1);
    }

    #[test]
    fn parses_new_blocks() {
        let src = "+++\nnumber = 2\nlabel = \"2A\"\ntitle = \"T\"\nquestion = \"Q\"\nobjectives = [\"o\"]\n+++\n~~~example Worked\nBody *md*.\n~~~\n~~~evidence\nNew.\n~~~\n~~~problems\n[[p]]\ntier = \"B\"\nprompt = \"p\"\nsolution = \"s\"\n~~~\n~~~figure\nfile = \"a.gif\"\nalt = \"a\"\ncaption = \"c\"\n~~~\n";
        let c = parse_chapter("x", Part::Main, src).unwrap();
        assert_eq!(c.meta.label(), "2A");
        assert_eq!(c.blocks.len(), 4);
        assert!(matches!(&c.blocks[0], Block::Example { title, .. } if title == "Worked"));
        assert!(matches!(&c.blocks[2], Block::Problems(p) if p.items[0].tier == "B"));
    }

    #[test]
    fn rejects_bad_tier() {
        let src = "+++\nnumber = 1\ntitle = \"T\"\nquestion = \"Q\"\n+++\n~~~problems\n[[p]]\ntier = \"D\"\nprompt = \"p\"\nsolution = \"s\"\n~~~\n";
        assert!(parse_chapter("x", Part::Main, src).is_err());
    }
}

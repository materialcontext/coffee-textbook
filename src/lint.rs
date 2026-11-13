//! Checks that run at build time. All pure: text in, findings out.
//!
//! - `prose`: style rules derived from published catalogues of machine-prose
//!   tells (Wikipedia "Signs of AI writing"; Pangram corpus measurements).
//! - `compat`: markup that a 2006-era browser cannot handle.
//! - `glyphs`: characters outside the WGL4 repertoire that Windows XP-era
//!   core fonts are guaranteed to contain.

#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    pub rule: &'static str,
    pub line: usize,
    pub excerpt: String,
}

/// Phrases to avoid in book prose. Lower-case; matched case-insensitively.
const PHRASES: &[(&str, &str)] = &[
    ("delve", "stock vocabulary"),
    ("tapestry", "stock vocabulary"),
    ("testament to", "significance inflation"),
    ("pivotal", "significance inflation"),
    ("crucial", "significance inflation"),
    ("landscape", "stock vocabulary"),
    ("in today's", "stock opener"),
    ("ever-evolving", "stock vocabulary"),
    ("navigate the", "stock vocabulary"),
    ("it's important to note", "filler"),
    ("it is important to note", "filler"),
    ("it's worth noting", "filler"),
    ("in conclusion", "summary closer"),
    ("in summary", "summary closer"),
    ("overall,", "summary closer"),
    ("not just", "negative parallelism"),
    ("not only", "negative parallelism"),
    ("isn't just", "negative parallelism"),
    ("game-changer", "puffery"),
    ("seamless", "puffery"),
    ("unlock", "puffery"),
    ("embark", "puffery"),
    ("journey", "puffery"),
    ("vibrant", "puffery"),
    ("rich tapestry", "puffery"),
    ("let's dive", "chatbot register"),
    ("great question", "chatbot register"),
    (" simply ", "condescending filler"),
    ("obviously", "condescending filler"),
    ("of course", "condescending filler"),
    ("it can be seen that", "filler"),
    ("in other words", "filler"),
];

/// Blocks whose body is prose and is linted like the main text.
const PROSE_BLOCKS: &[&str] = &["example", "evidence", "misconception", "deeper"];

/// Longest acceptable sentence, in words.
pub const MAX_SENTENCE: usize = 40;

/// Prose rules. Skips fenced code and data blocks; lints prose blocks.
pub fn prose(src: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut skip = false;
    let mut in_block = false;
    let mut para = String::new();
    let mut para_line = 0;
    let flush = |para: &mut String, line: usize, out: &mut Vec<Finding>| {
        for sentence in sentences(para) {
            let n = sentence.split_whitespace().count();
            if n > MAX_SENTENCE {
                out.push(Finding { rule: "long sentence", line, excerpt: format!("{n} words: {}", excerpt(&sentence)) });
            }
        }
        para.clear();
    };
    let mut front = false;
    for (i, line) in src.lines().enumerate() {
        let t = line.trim_start();
        if t.trim_end() == "+++" {
            front = !front;
            continue;
        }
        if front {
            continue;
        }
        if t.starts_with("```") {
            skip = !skip;
            continue;
        }
        if let Some(h) = t.strip_prefix("~~~") {
            flush(&mut para, para_line, &mut out);
            if in_block {
                in_block = false;
                skip = false;
            } else {
                in_block = true;
                let kind = h.split_whitespace().next().unwrap_or("");
                skip = !PROSE_BLOCKS.contains(&kind);
            }
            continue;
        }
        if t.starts_with("## References") || t.starts_with("## Further reading") {
            flush(&mut para, para_line, &mut out);
            break;
        }
        if skip || t.starts_with("+++") || t.starts_with('|') || t.starts_with('#') {
            continue;
        }
        let item = t.starts_with("- ") || t.split_once(". ").is_some_and(|(n, _)| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
        if t.is_empty() || item {
            flush(&mut para, para_line, &mut out);
        }
        if !t.is_empty() {
            if para.is_empty() {
                para_line = i + 1;
            }
            para.push(' ');
            para.push_str(t);
        }
        let lower = line.to_lowercase();
        if line.contains('\u{2014}') {
            out.push(Finding { rule: "em dash", line: i + 1, excerpt: excerpt(line) });
        }
        if line.contains("**") {
            out.push(Finding { rule: "bold in prose", line: i + 1, excerpt: excerpt(line) });
        }
        for (p, why) in PHRASES {
            if lower.contains(p) {
                out.push(Finding { rule: why, line: i + 1, excerpt: format!("`{p}`: {}", excerpt(line)) });
            }
        }
    }
    flush(&mut para, para_line, &mut out);
    out
}

/// Split plain text into sentences, ignoring markup. A sentence ends at
/// `.`, `?` or `!` followed by a space and an upper-case letter or digit.
pub fn sentences(text: &str) -> Vec<String> {
    let mut plain = String::new();
    let mut in_tag = false;
    for c in text.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => plain.push(c),
            _ => {}
        }
    }
    let chars: Vec<char> = plain.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    for (i, &c) in chars.iter().enumerate() {
        cur.push(c);
        let closer = matches!(c, '"' | ')') && i > 0 && matches!(chars[i - 1], '.' | '?' | '!');
        let end = (matches!(c, '.' | '?' | '!') || closer)
            && chars.get(i + 1) == Some(&' ')
            && chars.get(i + 2).is_some_and(|n| n.is_uppercase() || n.is_ascii_digit() || *n == '"' || *n == '{');
        let abbrev = ["e.g.", "i.e.", "et al.", "approx.", "Fig.", "Eq."].iter().any(|a| cur.ends_with(a));
        if end && !abbrev {
            out.push(std::mem::take(&mut cur).trim().to_string());
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

fn excerpt(s: &str) -> String {
    let s = s.trim();
    if s.chars().count() > 70 {
        format!("{}...", s.chars().take(70).collect::<String>())
    } else {
        s.to_string()
    }
}

/// Elements a 2006-era browser does not understand.
const HTML5_ELEMENTS: &[&str] = &[
    "section", "nav", "article", "aside", "header", "footer", "main", "figure",
    "figcaption", "details", "summary", "canvas", "svg", "video", "audio", "mark",
    "time", "output", "meter", "progress",
];

/// Attribute fragments a 2006-era browser does not understand.
const HTML5_ATTRS: &[&str] = &[
    "type=\"number\"", "type=\"range\"", "type=\"email\"", " data-", "placeholder=",
    " required", " autofocus",
];

/// Scans only inside tags, so prose that mentions "required" is not flagged.
pub fn compat(html: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    for (i, line) in html.lines().enumerate() {
        for tag in line.split('<').skip(1).filter_map(|s| s.split('>').next()) {
            let name: String = tag
                .trim_start_matches('/')
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect::<String>()
                .to_ascii_lowercase();
            if HTML5_ELEMENTS.contains(&name.as_str()) {
                out.push(Finding { rule: "not HTML 4.01", line: i + 1, excerpt: format!("<{name}>") });
            }
            let bare = strip_quoted(tag);
            for a in HTML5_ATTRS {
                let hay = if a.starts_with("type=") { tag } else { bare.as_str() };
                if hay.contains(a) {
                    out.push(Finding { rule: "not HTML 4.01", line: i + 1, excerpt: a.trim().to_string() });
                }
            }
        }
    }
    out
}

/// Removes the contents of double-quoted attribute values, so alt text and
/// titles that happen to contain words like "required" are not mistaken for
/// attributes.
fn strip_quoted(tag: &str) -> String {
    let mut out = String::with_capacity(tag.len());
    let mut inside = false;
    for c in tag.chars() {
        if c == '"' {
            inside = !inside;
            out.push(c);
        } else if !inside {
            out.push(c);
        }
    }
    out
}

/// True if `c` is in (a practical subset of) the WGL4 character set.
pub fn in_wgl4(c: char) -> bool {
    let u = c as u32;
    matches!(u,
        0x09 | 0x0A | 0x0D | 0x20..=0x7E | 0xA0..=0x17F | 0x192 | 0x1FA..=0x1FF
        | 0x2C6 | 0x2C7 | 0x2C9 | 0x2D8..=0x2DD | 0x384..=0x3CE | 0x401..=0x45F
        | 0x2013..=0x2015 | 0x2017..=0x201E | 0x2020..=0x2022 | 0x2026 | 0x2030
        | 0x2032 | 0x2033 | 0x2039 | 0x203A | 0x203C | 0x203E | 0x2044 | 0x207F
        | 0x20A3 | 0x20A4 | 0x20A7 | 0x20AC | 0x2105 | 0x2113 | 0x2116 | 0x2122
        | 0x2126 | 0x212E | 0x215B..=0x215E | 0x2190..=0x2195 | 0x21A8 | 0x2202
        | 0x2206 | 0x220F | 0x2211 | 0x2212 | 0x2215 | 0x2219 | 0x221A | 0x221E
        | 0x221F | 0x2229 | 0x222B | 0x2248 | 0x2260 | 0x2261 | 0x2264 | 0x2265
        | 0x25A0..=0x25A1 | 0x25CB | 0x25CF)
}

pub fn glyphs(text: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        for c in line.chars() {
            if !in_wgl4(c) {
                out.push(Finding {
                    rule: "glyph outside WGL4",
                    line: i + 1,
                    excerpt: format!("U+{:04X} {c}", c as u32),
                });
            }
        }
    }
    out
}

/// Every bracketed citation [n] (or [n, m], [n-m]) must have an nth entry in
/// the numbered list under a "## References" heading. Markdown links [x](y)
/// and footnotes are ignored.
pub fn citations(src: &str) -> Vec<Finding> {
    let (body, refs) = match src.find("\n## References") {
        Some(i) => (&src[..i], &src[i..]),
        None => (src, ""),
    };
    let entries = refs
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
            !digits.is_empty() && t[digits.len()..].starts_with(". ")
        })
        .count();
    let mut out = Vec::new();
    let mut cited = std::collections::BTreeSet::new();
    for (n, line) in body.lines().enumerate() {
        let b = line.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if b[i] == b'[' {
                if let Some(rel) = line[i + 1..].find(']') {
                    let inner = &line[i + 1..i + 1 + rel];
                    let after = line[i + 2 + rel..].chars().next();
                    let is_cite = !inner.is_empty()
                        && inner.chars().all(|c| c.is_ascii_digit() || c == ',' || c == ' ' || c == '-')
                        && inner.chars().next().is_some_and(|c| c.is_ascii_digit())
                        && after != Some('(');
                    if is_cite {
                        let mut nums = Vec::new();
                        for part in inner.split(',') {
                            let mut ends = part.split('-').filter_map(|x| x.trim().parse::<usize>().ok());
                            match (ends.next(), ends.next()) {
                                (Some(a), Some(b)) if a <= b => nums.extend(a..=b),
                                (Some(a), _) => nums.push(a),
                                _ => {}
                            }
                        }
                        for k in nums {
                            cited.insert(k);
                            {
                                if k == 0 || k > entries {
                                    out.push(Finding {
                                        rule: "citation without reference",
                                        line: n + 1,
                                        excerpt: format!("[{k}] but {entries} references"),
                                    });
                                }
                            }
                        }
                    }
                    i += rel + 1;
                }
            }
            i += 1;
        }
    }
    for k in 1..=entries {
        if !cited.contains(&k) {
            out.push(Finding { rule: "reference never cited", line: 0, excerpt: format!("reference {k}") });
        }
    }
    out
}

/// Figures must be GIF or PNG (decodable by 2006-era browsers) and within budget.
pub fn figure(bytes: &[u8], budget: usize) -> Vec<Finding> {
    let mut out = Vec::new();
    let gif = bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a");
    let png = bytes.starts_with(&[0x89, b'P', b'N', b'G']);
    if !gif && !png {
        out.push(Finding { rule: "figure not GIF or PNG", line: 0, excerpt: String::new() });
    }
    if bytes.len() > budget {
        out.push(Finding { rule: "figure over budget", line: 0, excerpt: format!("{} bytes", bytes.len()) });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_sentences_and_prose_blocks() {
        let long = format!("{}.", vec!["word"; 45].join(" "));
        assert!(prose(&long).iter().any(|f| f.rule == "long sentence"));
        assert!(prose("~~~lab\nx = \"obviously\"\n~~~\n").is_empty());
        assert!(!prose("~~~example T\nThis is obviously true.\n~~~\n").is_empty());
        assert_eq!(sentences("A b. C d? 3 e. et al. Fine.").len(), 3);
    }

    #[test]
    fn citations_check_both_directions() {
        let src = "Text [1-2] and [4].\n\n## References\n\n1. A\n2. B\n3. C\n";
        let f = citations(src);
        assert!(f.iter().any(|x| x.rule == "citation without reference"));
        assert!(f.iter().any(|x| x.rule == "reference never cited" && x.excerpt == "reference 3"));
        assert!(!f.iter().any(|x| x.excerpt == "reference 2"));
    }

    #[test]
    fn compat_ignores_attribute_values() {
        assert!(compat("<img src=\"a.gif\" alt=\"heat required to roast\">").is_empty());
        assert!(!compat("<input name=\"x\" required>").is_empty());
        assert!(!compat("<input type=\"number\">").is_empty());
    }

    #[test]
    fn flags_em_dash_and_phrases() {
        let f = prose("This is pivotal \u{2014} truly.\n");
        assert!(f.iter().any(|x| x.rule == "em dash"));
        assert!(f.iter().any(|x| x.rule == "significance inflation"));
    }

    #[test]
    fn ignores_code_fences() {
        assert!(prose("```\ncrucial\n```\n").is_empty());
    }

    #[test]
    fn compat_flags_html5() {
        assert_eq!(compat("<section>").len(), 1);
        assert!(compat("<div class=\"x\">").is_empty());
    }

    #[test]
    fn glyph_check() {
        assert!(glyphs("CO2 \u{00B0}C \u{2248} \u{2192}").is_empty());
        assert_eq!(glyphs("CO\u{2082}").len(), 1);
    }

    #[test]
    fn citation_check() {
        let ok = "See [1] and [2, 3] and [a link](x) and [1-2].\n\n## References\n\n1. A\n2. B\n3. C\n";
        assert!(citations(ok).is_empty());
        let bad = "See [4].\n\n## References\n\n1. A\n";
        assert_eq!(citations(bad).iter().filter(|f| f.rule == "citation without reference").count(), 1);
    }

    #[test]
    fn figure_check() {
        assert!(figure(b"GIF89a....", 100).is_empty());
        assert_eq!(figure(b"JPEG", 100).len(), 1);
        assert_eq!(figure(&[b'G', b'I', b'F', b'8', b'9', b'a', 0, 0], 4).len(), 1);
    }
}

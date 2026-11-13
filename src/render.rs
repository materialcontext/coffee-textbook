//! Domain values to HTML 4.01 strings. Pure functions.
//!
//! Output rules (enforced by `lint::compat`):
//! - HTML 4.01 elements only; no HTML5 sectioning or form types.
//! - Every interactive element works as a plain form without script, and
//!   every quiz answer and problem solution is reachable through a link.

use crate::domain::{Block, Book, BookMeta, Chapter, DisplayEq, Equations, Figure, Lab, Part, Problems, Question, Quiz, Term};
use crate::widgets;
use pulldown_cmark::{html, Options, Parser};
use std::collections::{HashMap, HashSet};

pub fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            _ => o.push(c),
        }
    }
    o
}

pub fn markdown(src: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    let mut out = String::new();
    html::push_html(&mut out, Parser::new_ext(src, opts));
    out
}

/// Inline Markdown for short strings: strips the wrapping paragraph.
pub fn inline_md(src: &str) -> String {
    let h = markdown(src);
    let t = h.trim();
    t.strip_prefix("<p>")
        .and_then(|s| s.strip_suffix("</p>"))
        .filter(|s| !s.contains("<p>"))
        .unwrap_or(t)
        .to_string()
}

pub fn quiz_id(slug: &str, n: usize) -> String {
    format!("{slug}-q{n}")
}

pub fn solutions_file(slug: &str) -> String {
    format!("{slug}-solutions.html")
}

// ---------------------------------------------------------------- quizzes

pub fn quiz(slug: &str, n: usize, q: &Quiz) -> String {
    let id = quiz_id(slug, n);
    let title = q.title.as_deref().unwrap_or("Self-check");
    let mut h = format!(
        "<div class=\"quiz\">\n<h3>{}</h3>\n<form class=\"quiz-form\" id=\"{id}\" action=\"#answers\">\n",
        esc(title)
    );
    for (i, question) in q.questions.iter().enumerate() {
        let qid = format!("{id}-{i}");
        h.push_str(&format!("<div class=\"q\">\n<p class=\"prompt\">{}. {}</p>\n", i + 1, inline_md(question.prompt())));
        match question {
            Question::Choice { options, answer, .. } => {
                h.push_str("<p class=\"opts\">\n");
                for (j, opt) in options.iter().enumerate() {
                    h.push_str(&format!(
                        "<input type=\"radio\" name=\"{qid}\" id=\"{qid}-{j}\" value=\"{j}\"> <label for=\"{qid}-{j}\">{}</label><br>\n",
                        inline_md(opt)
                    ));
                }
                h.push_str(&format!(
                    "</p>\n<div><input type=\"hidden\" name=\"{qid}-kind\" value=\"choice\"><input type=\"hidden\" name=\"{qid}-ans\" value=\"{answer}\"></div>\n"
                ));
            }
            Question::Numeric { answer, tolerance, unit, .. } => {
                h.push_str(&format!(
                    "<p class=\"opts\"><label for=\"{qid}\">Answer:</label> <input type=\"text\" size=\"8\" name=\"{qid}\" id=\"{qid}\"> {}</p>\n<div><input type=\"hidden\" name=\"{qid}-kind\" value=\"numeric\"><input type=\"hidden\" name=\"{qid}-ans\" value=\"{answer}\"><input type=\"hidden\" name=\"{qid}-tol\" value=\"{tolerance}\"></div>\n",
                    esc(unit)
                ));
            }
        }
        h.push_str(&format!("<p class=\"fb\" id=\"{qid}-fb\"></p>\n</div>\n"));
    }
    h.push_str(&format!(
        "<p><input type=\"hidden\" name=\"quiz-key\" value=\"{id}\"><input type=\"hidden\" name=\"quiz-n\" value=\"{}\"><input type=\"button\" class=\"check\" value=\"Check answers\"> <a href=\"#ans-{id}\">Answers</a></p>\n</form>\n</div>\n",
        q.questions.len()
    ));
    h
}

pub fn answer_key(slug: &str, quizzes: &[&Quiz]) -> String {
    if quizzes.is_empty() {
        return String::new();
    }
    let mut h = String::from("<div class=\"answers\">\n<h2 id=\"answers\">Answers to self-checks</h2>\n");
    for (n, q) in quizzes.iter().enumerate() {
        let id = quiz_id(slug, n);
        h.push_str(&format!("<h3 id=\"ans-{id}\">{}</h3>\n<ol>\n", esc(q.title.as_deref().unwrap_or("Self-check"))));
        for (i, question) in q.questions.iter().enumerate() {
            let key = match question {
                Question::Choice { options, answer, .. } => inline_md(&options[*answer]),
                Question::Numeric { answer, unit, .. } => format!("{answer} {}", esc(unit)),
            };
            h.push_str(&format!(
                "<li><span class=\"key\">{key}.</span> <span id=\"{id}-{i}-ex\">{}</span></li>\n",
                inline_md(question.explain())
            ));
        }
        h.push_str("</ol>\n");
    }
    h.push_str("</div>\n");
    h
}

// ---------------------------------------------------------------- new blocks

/// Labels that open the paragraphs of a worked example.
pub const STEPS: &[&str] = &["Problem.", "Plan.", "Solution.", "Check.", "Your turn.", "Answer."];

pub fn example(title: &str, body: &str) -> String {
    let mut h = markdown(body);
    for l in STEPS {
        h = h.replace(&format!("<p>{l}"), &format!("<p><span class=\"step\">{l}</span>"));
    }
    format!("<div class=\"example\">\n<p class=\"boxhead\">Worked example: {}</p>\n{h}</div>\n", inline_md(title))
}

pub fn misconception(body: &str) -> String {
    format!("<div class=\"misc\">\n<p class=\"boxhead\">Common misconception</p>\n{}</div>\n", markdown(body))
}

pub fn deeper(title: &str, body: &str) -> String {
    let t = if title.is_empty() { String::new() } else { format!(": {}", inline_md(title)) };
    format!("<div class=\"deeper\">\n<p class=\"boxhead\">Going further{t}</p>\n{}</div>\n", markdown(body))
}

pub fn display_eq(e: &DisplayEq, number: &str) -> String {
    let mut h = format!(
        "<table class=\"deq\" summary=\"Equation {number}\"><tr><td class=\"f\">{}</td><td class=\"n\" id=\"eq-{}\">({number})</td></tr></table>\n",
        e.formula,
        esc(&e.id)
    );
    if !e.symbols.is_empty() {
        h.push_str("<table class=\"where\" summary=\"Symbols\">\n");
        for (i, row) in e.symbols.iter().enumerate() {
            let lead = if i == 0 { "where" } else { "" };
            let unit = row.get(2).filter(|u| !u.is_empty()).map(|u| format!(" ({u})")).unwrap_or_default();
            h.push_str(&format!("<tr><td class=\"w\">{lead}</td><td class=\"s\">{}</td><td>{}{unit}</td></tr>\n", row[0], inline_md(&row[1])));
        }
        h.push_str("</table>\n");
    }
    h
}

// ---------------------------------------------------------------- cross-references and terms

/// A term defined in the text with `{{term}}` or `{{glossary key|text as printed}}`.
#[derive(Debug, Clone)]
pub struct DefinedTerm {
    pub text: String,
    pub key: String,
    pub anchor: String,
    /// The sentence that defines the term (and the next one if it is short), as HTML.
    pub context: String,
}

/// Normalised glossary key: tags removed, parenthetical tail removed, lower case.
pub fn term_key(s: &str) -> String {
    let mut plain = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => plain.push(c),
            _ => {}
        }
    }
    let cut = plain.split(" (").next().unwrap_or("");
    cut.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// Per-chapter numbering of equations and figures, and the terms defined so far.
pub struct Ctx {
    eqs: HashMap<String, String>,
    figs: HashMap<String, String>,
    pub terms: Vec<DefinedTerm>,
    seen: HashSet<String>,
}

impl Ctx {
    pub fn new(c: &Chapter) -> Result<Ctx, String> {
        let label = c.meta.label();
        let (mut eqs, mut figs) = (HashMap::new(), HashMap::new());
        let (mut ne, mut nf) = (0, 0);
        for b in &c.blocks {
            match b {
                Block::Eq(e) => {
                    ne += 1;
                    if eqs.insert(e.id.clone(), format!("{label}.{ne}")).is_some() {
                        return Err(format!("{}: duplicate equation id `{}`", c.slug, e.id));
                    }
                }
                Block::Figure(f) => {
                    nf += 1;
                    if let Some(id) = &f.id {
                        if figs.insert(id.clone(), format!("{label}.{nf}")).is_some() {
                            return Err(format!("{}: duplicate figure id `{id}`", c.slug));
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(Ctx { eqs, figs, terms: Vec::new(), seen: HashSet::new() })
    }

    pub fn eq_number(&self, id: &str) -> &str {
        &self.eqs[id]
    }

    /// Replace `{{term}}`, `{{key|text}}`, `{eq:id}` and `{fig:id}`.
    pub fn resolve(&mut self, s: &str) -> Result<String, String> {
        let mut out = String::with_capacity(s.len());
        let mut rest = s;
        let whole = s;
        loop {
            let next = [rest.find("{{"), rest.find("{eq:"), rest.find("{fig:")].into_iter().flatten().min();
            let Some(i) = next else {
                out.push_str(rest);
                return Ok(out);
            };
            out.push_str(&rest[..i]);
            let tail = &rest[i..];
            if let Some(t) = tail.strip_prefix("{{") {
                let end = t.find("}}").ok_or("unclosed {{ in text")?;
                let inner = &t[..end];
                let (name, text) = inner.split_once('|').unwrap_or((inner, inner));
                let key = term_key(name);
                let anchor = format!("t-{}", anchor(&key));
                if self.seen.insert(key.clone()) {
                    out.push_str(&format!("<dfn id=\"{anchor}\">{text}</dfn>"));
                    let at = whole.len() - tail.len();
                    let context = self.plain(&sentence_around(whole, at, at + 2 + end + 2));
                    // Lists show the glossary form of the term; the text shows the sentence form.
                    self.terms.push(DefinedTerm { text: name.to_string(), key, anchor, context: inline_md(&context) });
                } else {
                    out.push_str(&format!("<dfn>{text}</dfn>"));
                }
                rest = &t[end + 2..];
            } else {
                let end = tail.find('}').ok_or("unclosed reference in text")?;
                let inner = &tail[1..end];
                let (kind, id) = inner.split_once(':').unwrap_or(("", ""));
                let (map, word) = if kind == "eq" { (&self.eqs, "Equation") } else { (&self.figs, "Figure") };
                let num = map.get(id).ok_or_else(|| format!("unknown reference {{{inner}}}"))?;
                if out.trim_end().ends_with(word) || out.trim_end().ends_with(&format!("{word}s")) {
                    return Err(format!("`{word} {{{inner}}}` would print the word twice; write `{{{inner}}}` alone"));
                }
                out.push_str(&format!("<a href=\"#{kind}-{id}\">{word} {num}</a>"));
                rest = &tail[end + 1..];
            }
        }
    }
}

impl Ctx {
    /// Markup-free version of a source fragment: terms become their text,
    /// references become "Equation 2A.3" without links.
    fn plain(&self, s: &str) -> String {
        let mut out = String::new();
        let mut rest = s;
        while let Some(i) = [rest.find("{{"), rest.find("{eq:"), rest.find("{fig:")].into_iter().flatten().min() {
            out.push_str(&rest[..i]);
            let tail = &rest[i..];
            if let Some(t) = tail.strip_prefix("{{") {
                let Some(end) = t.find("}}") else { break };
                let inner = &t[..end];
                out.push_str(inner.split_once('|').map(|(_, x)| x).unwrap_or(inner));
                rest = &t[end + 2..];
            } else {
                let Some(end) = tail.find('}') else { break };
                let inner = &tail[1..end];
                let (kind, id) = inner.split_once(':').unwrap_or(("", ""));
                let (map, word) = if kind == "eq" { (&self.eqs, "Equation") } else { (&self.figs, "Figure") };
                out.push_str(&format!("{word} {}", map.get(id).map(String::as_str).unwrap_or("?")));
                rest = &tail[end + 1..];
            }
        }
        out.push_str(rest);
        out
    }
}

/// How a term is listed in the index and on flashcards: lower case, unless it
/// is named after someone or is an acronym. A name is recognised by a later
/// capital ("Beer-Lambert law"), a possessive ("Fick's law"), or a capital in
/// the middle of a sentence somewhere in `text`, the book's prose ("the Maillard reaction").
pub fn index_form(name: &str, text: &str) -> String {
    let first = name.split([' ', '-', '\'']).next().unwrap_or("");
    let mut cs = first.chars();
    let (Some(a), b) = (cs.next(), cs.next()) else { return name.to_string() };
    let plain = name.split(" (").next().unwrap_or(name);
    let first_word = plain.split(' ').next().unwrap_or("");
    let named = first_word.split('-').skip(1).any(|w| w.starts_with(char::is_uppercase)) || plain.contains("'s ");
    if !a.is_uppercase() || b.is_some_and(|b| !b.is_lowercase()) || named {
        return name.to_string();
    }
    let proper = text.match_indices(first).any(|(i, _)| {
        let before = text[..i].trim_end_matches([' ', '*', '_', '(', '{']);
        let after = text[i + first.len()..].chars().next();
        let whole_word = !after.is_some_and(|c| c.is_alphanumeric());
        // The word before must be an ordinary lower-case word, not the end of a title.
        let prev = before.rsplit(|c: char| !c.is_alphabetic()).next().unwrap_or("");
        let lower_word = !prev.is_empty() && prev.chars().all(char::is_lowercase) && before.ends_with(prev);
        // "A. A. Griffith": an initial, not the end of a sentence.
        let b = before.as_bytes();
        let initial = b.len() >= 3 && b[b.len() - 1] == b'.' && b[b.len() - 2].is_ascii_uppercase() && b[b.len() - 3] == b' ';
        whole_word && (lower_word || initial)
    });
    if proper {
        return name.to_string();
    }
    let mut out = a.to_lowercase().collect::<String>();
    out.push_str(&name[a.len_utf8()..]);
    out
}

/// The sentence of `s` containing bytes `from..to`, plus the following sentence
/// when the first is short. Sentences end at ". " (or "? ", "! ") or a blank line.
pub fn sentence_around(s: &str, from: usize, to: usize) -> String {
    let before = &s[..from];
    let start = [before.rfind(". "), before.rfind("? "), before.rfind("! "), before.rfind("\n\n"), before.rfind("\n- "), before.rfind("\n1. ")]
        .into_iter()
        .flatten()
        .max()
        .map(|i| i + 2)
        .unwrap_or(0);
    let end_after = |from: usize| -> usize {
        let rest = &s[from.min(s.len())..];
        [rest.find(". "), rest.find(".\n"), rest.find("\n\n")]
            .into_iter()
            .flatten()
            .min()
            .map(|i| from + i + 1)
            .unwrap_or(s.len())
    };
    let mut end = end_after(to.min(s.len()));
    if s[start..end].split_whitespace().count() < 14 && end < s.len() {
        end = end_after(end + 1);
    }
    s[start..end.min(s.len())].trim().trim_start_matches("- ").replace('\n', " ")
}

pub fn key_terms(terms: &[DefinedTerm]) -> String {
    if terms.is_empty() {
        return String::new();
    }
    let mut t: Vec<&DefinedTerm> = terms.iter().collect();
    t.sort_by_key(|x| x.key.clone());
    let links: Vec<String> = t.iter().map(|x| format!("<a href=\"#{}\">{}</a>", x.anchor, x.text)).collect();
    format!("<h2 id=\"key-terms\">Key terms</h2>\n<p class=\"kt\">{}.</p>\n", links.join("; "))
}

pub fn evidence(body: &str) -> String {
    format!("<div class=\"evidence\">\n<p class=\"boxhead\">Status of the evidence</p>\n{}</div>\n", markdown(body))
}

pub fn equations(e: &Equations) -> String {
    let mut h = String::from("<table class=\"eqs\" summary=\"Key equations\">\n<tr><th>Quantity</th><th>Equation</th><th>Notes</th></tr>\n");
    for q in &e.items {
        h.push_str(&format!("<tr><td>{}</td><td class=\"eq\">{}</td><td>{}</td></tr>\n", inline_md(&q.name), q.formula, inline_md(&q.note)));
    }
    h.push_str("</table>\n");
    h
}

pub fn figure(f: &Figure, n: usize, label: &str) -> String {
    let id = f.id.as_deref().map(|i| format!(" id=\"fig-{}\"", esc(i))).unwrap_or_default();
    format!(
        "<div class=\"figure\"{id}>\n<img src=\"figures/{}\" alt=\"{}\">\n<p class=\"caption\">Figure {label}.{n}. {}</p>\n</div>\n",
        esc(&f.file),
        esc(&f.alt),
        inline_md(&f.caption)
    )
}

pub fn lab(l: &Lab) -> String {
    let list = |items: &[String], tag: &str| {
        let mut h = format!("<{tag}>\n");
        for i in items {
            h.push_str(&format!("<li>{}</li>\n", inline_md(i)));
        }
        h.push_str(&format!("</{tag}>\n"));
        h
    };
    let mut h = format!(
        "<div class=\"lab\">\n<h3 id=\"lab-{}\">Lab {}: {}</h3>\n<p><span class=\"lbl\">Aim.</span> {}</p>\n",
        esc(&l.id), esc(&l.id), esc(&l.title), inline_md(&l.aim)
    );
    if !l.safety.is_empty() {
        h.push_str("<p class=\"lbl\">Safety</p>\n");
        h.push_str(&list(&l.safety, "ul"));
    }
    h.push_str("<p class=\"lbl\">Materials</p>\n");
    h.push_str(&list(&l.materials, "ul"));
    h.push_str("<p class=\"lbl\">Method</p>\n");
    h.push_str(&list(&l.method, "ol"));
    h.push_str("<p class=\"lbl\">Data sheet</p>\n<table class=\"datasheet\" summary=\"Blank data sheet\">\n<tr>");
    for c in &l.columns {
        h.push_str(&format!("<th>{}</th>", inline_md(c)));
    }
    h.push_str("</tr>\n");
    for _ in 0..l.rows {
        h.push_str("<tr>");
        for _ in &l.columns {
            h.push_str("<td>&nbsp;</td>");
        }
        h.push_str("</tr>\n");
    }
    h.push_str("</table>\n<p class=\"lbl\">Analysis</p>\n");
    h.push_str(&list(&l.analysis, "ol"));
    if let Some(e) = &l.expected {
        h.push_str(&format!("<p><span class=\"lbl\">What to expect.</span> {}</p>\n", inline_md(e)));
    }
    h.push_str("</div>\n");
    h
}

/// Problems render in the chapter; solutions go to the chapter's solutions page.
pub fn problems(slug: &str, set: usize, start: usize, p: &Problems) -> (String, String) {
    let file = solutions_file(slug);
    let mut q = String::from("<div class=\"problems\">\n<ol class=\"plist\" start=\"");
    q.push_str(&format!("{}\">\n", start + 1));
    let mut s = String::new();
    for (i, item) in p.items.iter().enumerate() {
        let n = start + i + 1;
        let tag = match &item.section {
            Some(sec) => format!("{}, Section {}", esc(&item.tier), esc(sec)),
            None => esc(&item.tier),
        };
        q.push_str(&format!(
            "<li id=\"p{set}-{n}\"><span class=\"tier\">[{tag}]</span> {} <a class=\"sol\" href=\"{file}#s{n}\">Solution</a></li>\n",
            inline_md(&item.prompt)
        ));
        s.push_str(&format!(
            "<div class=\"solution\">\n<h3 id=\"s{n}\">Problem {n} <span class=\"tier\">[{}]</span></h3>\n<p class=\"restate\">{}</p>\n{}</div>\n",
            esc(&item.tier),
            inline_md(&item.prompt),
            markdown(&item.solution)
        ));
    }
    q.push_str("</ol>\n<p class=\"small\">Tiers: A recall, B calculation, C analysis or design.</p>\n</div>\n");
    (q, s)
}

pub fn objectives(c: &Chapter) -> String {
    let mut h = String::new();
    if !c.meta.objectives.is_empty() {
        h.push_str("<div class=\"objectives\">\n<p class=\"boxhead\">After this chapter you can</p>\n<ul>\n");
        for o in &c.meta.objectives {
            h.push_str(&format!("<li>{}</li>\n", inline_md(o)));
        }
        h.push_str("</ul>\n");
        if !c.meta.prerequisites.is_empty() {
            h.push_str("<p class=\"small\">Prerequisites: ");
            h.push_str(&c.meta.prerequisites.iter().map(|p| inline_md(p)).collect::<Vec<_>>().join("; "));
            h.push_str(".</p>\n");
        }
        h.push_str("</div>\n");
    }
    h
}

/// Rendered chapter: body HTML and solutions HTML (empty if no problems).
pub struct Rendered {
    pub body: String,
    pub solutions: String,
    pub terms: Vec<DefinedTerm>,
}

const KT_MARK: &str = "\u{0}KEYTERMS\u{0}";

pub fn chapter_body(c: &Chapter) -> Result<Rendered, String> {
    let fail = |e: String| format!("{}: {e}", c.slug);
    let mut ctx = Ctx::new(c)?;
    let mut h = objectives(c);
    let mut sol = String::new();
    let mut quizzes = Vec::new();
    let (mut figs, mut probs, mut sets, mut placed) = (0, 0, 0, false);
    for (bi, b) in c.blocks.iter().enumerate() {
        match b {
            Block::Prose(md) => h.push_str(&markdown(&ctx.resolve(md).map_err(fail)?)),
            Block::Quiz(q) => {
                h.push_str(&quiz(&c.slug, quizzes.len(), q));
                quizzes.push(q);
            }
            Block::Widget(w) => {
                let id = format!("{}-w{bi}", c.slug);
                h.push_str(&widgets::render(&id, w).map_err(|e| fail(e.to_string()))?);
            }
            Block::Example { title, body } => h.push_str(&example(title, &ctx.resolve(body).map_err(fail)?)),
            Block::Evidence(body) => h.push_str(&evidence(&ctx.resolve(body).map_err(fail)?)),
            Block::Misconception(body) => h.push_str(&misconception(&ctx.resolve(body).map_err(fail)?)),
            Block::Deeper { title, body } => h.push_str(&deeper(title, &ctx.resolve(body).map_err(fail)?)),
            Block::Equations(e) => h.push_str(&equations(e)),
            Block::Eq(e) => {
                let n = ctx.eq_number(&e.id).to_string();
                h.push_str(&display_eq(e, &n));
            }
            Block::Figure(f) => {
                figs += 1;
                let mut f = f.clone();
                f.caption = ctx.resolve(&f.caption).map_err(fail)?;
                h.push_str(&figure(&f, figs, &c.meta.label()));
            }
            Block::Lab(l) => h.push_str(&lab(l)),
            Block::Problems(p) => {
                let mut p = p.clone();
                for item in &mut p.items {
                    item.prompt = ctx.resolve(&item.prompt).map_err(fail)?;
                    item.solution = ctx.resolve(&item.solution).map_err(fail)?;
                }
                let (q, s) = problems(&c.slug, sets, probs, &p);
                h.push_str(&q);
                sol.push_str(&s);
                probs += p.items.len();
                sets += 1;
            }
            Block::KeyTerms => {
                h.push_str(KT_MARK);
                placed = true;
            }
        }
    }
    let kt = key_terms(&ctx.terms);
    if placed {
        h = h.replace(KT_MARK, &kt);
    } else {
        h.push_str(&kt);
    }
    h.push_str(&answer_key(&c.slug, &quizzes));
    // Cross-references in solutions point back to the chapter page.
    let sol = sol.replace("href=\"#", &format!("href=\"{}.html#", c.slug));
    Ok(Rendered { body: h, solutions: sol, terms: ctx.terms })
}

// ---------------------------------------------------------------- pages

pub struct Nav<'a> {
    pub prev: Option<(&'a str, &'a str)>,
    pub next: Option<(&'a str, &'a str)>,
}

fn nav_bar(nav: &Nav, class: &str) -> String {
    let link = |l: Option<(&str, &str)>, rel: &str, label: &str| match l {
        Some((href, title)) => format!("<a href=\"{href}\" rel=\"{rel}\" title=\"{}\">{label}</a>", esc(title)),
        None => format!("<span class=\"off\">{label}</span>"),
    };
    format!(
        "<div class=\"nav {class}\">{} | <a href=\"index.html\">Contents</a> | <a href=\"glossary.html\">Glossary</a> | {}</div>\n",
        link(nav.prev, "prev", "&lt; Previous"),
        link(nav.next, "next", "Next &gt;")
    )
}

pub fn heading(c: &Chapter) -> String {
    match c.part {
        Part::Front => c.meta.title.clone(),
        Part::Main => format!("{}. {}", c.meta.label(), c.meta.title),
        Part::Appendix => format!("Appendix {}. {}", c.meta.label(), c.meta.title),
    }
}

pub fn page(meta: &BookMeta, title: &str, lead: Option<&str>, body: &str, nav: &Nav) -> String {
    let lead = lead.map(|l| format!("<p class=\"lead\">{}</p>\n", esc(l))).unwrap_or_default();
    format!(
        "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">\n\
<!-- saved from url=(0014)about:internet -->\n\
<html lang=\"{lang}\">\n<head>\n\
<meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\">\n\
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
<title>{title} - {book}</title>\n\
<link rel=\"stylesheet\" type=\"text/css\" href=\"book.css\">\n\
<link rel=\"stylesheet\" type=\"text/css\" href=\"dark.css\" media=\"screen and (prefers-color-scheme: dark)\">\n\
<meta name=\"color-scheme\" content=\"light dark\">\n\
</head>\n<body>\n<div class=\"page\">\n{top}\
<h1>{title}</h1>\n{lead}\
<noscript><p class=\"note\">Script is off. Calculators show their formulas; answers and solutions are linked from each question.</p></noscript>\n\
{body}\n{bottom}\
<p class=\"foot\">{foot}</p>\n\
</div>\n<script type=\"text/javascript\" src=\"book.js\"></script>\n</body>\n</html>\n",
        lang = esc(&meta.lang),
        title = esc(title),
        book = esc(&meta.title),
        top = nav_bar(nav, "top"),
        bottom = nav_bar(nav, "bottom"),
        foot = footer(meta, ".html"),
    )
}

/// The copyright, version and licence line. `ext` is the link extension of the
/// edition (".html" for the web book, ".xhtml" for the EPUB).
pub fn footer(meta: &BookMeta, ext: &str) -> String {
    let version = meta
        .version
        .as_deref()
        .map(|v| format!(", version {} (<a href=\"changes{ext}\">changes and errata</a>)", esc(v)))
        .unwrap_or_default();
    let licensee = match &meta.buyer {
        Some(b) => format!("Licensed to {} for personal use", esc(b)),
        None => "Licensed for personal use".to_string(),
    };
    format!(
        "{}, {} edition{version}. &copy; {} {}. {licensee}; see <a href=\"license{ext}\">licence</a>.",
        esc(&meta.title),
        esc(&meta.edition),
        meta.year,
        esc(&meta.license_holder)
    )
}

pub fn contents(book: &Book) -> String {
    let m = &book.meta;
    let mut h = String::new();
    if let Some(s) = &m.subtitle {
        h.push_str(&format!("<p class=\"lead\">{}</p>\n", esc(s)));
    }
    let publisher = m.publisher.as_deref().map(|p| format!(", {}", esc(p))).unwrap_or_default();
    h.push_str(&format!("<p class=\"byline\">{}. {} edition{publisher}, {}.</p>\n", esc(&m.author), esc(&m.edition), m.year));
    let list = |h: &mut String, cs: &[Chapter]| {
        h.push_str("<ul class=\"toc\">\n");
        for c in cs {
            let lab = match c.part {
                Part::Front => String::new(),
                Part::Main => format!("{}. ", c.meta.label()),
                Part::Appendix => format!("{}. ", c.meta.label()),
            };
            h.push_str(&format!(
                "<li><a href=\"{}.html\">{lab}{}</a><br><span class=\"q\">{}</span></li>\n",
                c.slug,
                esc(&c.meta.title),
                esc(&c.meta.question)
            ));
        }
        h.push_str("</ul>\n");
    };
    h.push_str("<h2>Before you start</h2>\n");
    list(&mut h, &book.front);
    h.push_str("<h2>Chapters</h2>\n");
    list(&mut h, &book.chapters);
    if !book.appendices.is_empty() {
        h.push_str("<h2>Foundations appendices</h2>\n");
        list(&mut h, &book.appendices);
    }
    h.push_str("<h2>Reference</h2>\n<ul class=\"toc\">\n<li><a href=\"terms.html\">Index of terms</a></li>\n<li><a href=\"glossary.html\">Glossary</a></li>\n<li><a href=\"print.html\">Whole book on one page (for printing)</a></li>\n<li><a href=\"flashcards.txt\">Flashcards</a> <span class=\"small\">(every defined term and self-check question, as a text file that Anki and similar programs import)</span></li>\n<li><a href=\"changes.html\">Changes and errata</a></li>\n<li><a href=\"license.html\">Licence</a></li>\n</ul>\n");
    h
}

/// `defs` maps a normalised term key to (link, place name) of its definition in the text.
pub fn glossary(terms: &[Term], defs: &HashMap<String, (String, String)>) -> String {
    let mut sorted: Vec<&Term> = terms.iter().collect();
    sorted.sort_by_key(|t| term_key(&t.term));
    let mut h = String::from("<dl class=\"gloss\">\n");
    for t in sorted {
        let at = defs
            .get(&term_key(&t.term))
            .map(|(href, place)| format!(" <span class=\"small\">(<a href=\"{href}\">{}</a>)</span>", esc(place)))
            .unwrap_or_default();
        h.push_str(&format!(
            "<dt id=\"g-{}\">{}</dt>\n<dd>{}{at}</dd>\n",
            anchor(&term_key(&t.term)),
            t.term,
            inline_md(&t.definition)
        ));
    }
    h.push_str("</dl>\n");
    h
}

pub fn anchor(s: &str) -> String {
    s.chars().map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '-' }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Problem;

    #[test]
    fn escapes_markup() {
        assert_eq!(esc("<a & \"b\">"), "&lt;a &amp; &quot;b&quot;&gt;");
    }

    #[test]
    fn inline_strips_paragraph() {
        assert_eq!(inline_md("*x*"), "<em>x</em>");
    }

    #[test]
    fn quiz_has_hidden_keys_and_link() {
        let q: Quiz = toml::from_str("[[q]]\ntype=\"numeric\"\nprompt=\"p\"\nanswer=2.5\ntolerance=0.1\nunit=\"%\"\nexplain=\"e\"\n").unwrap();
        let h = quiz("ch02", 0, &q);
        assert!(h.contains("name=\"ch02-q0-0-ans\" value=\"2.5\""));
        assert!(answer_key("ch02", &[&q]).contains("id=\"ch02-q0-0-ex\""));
    }

    #[test]
    fn problems_number_continuously_and_link_solutions() {
        let p = Problems { items: vec![Problem { tier: "B".into(), section: Some("3".into()), prompt: "x".into(), solution: "y".into() }] };
        let (q, s) = problems("ch02a", 1, 4, &p);
        assert!(q.contains("start=\"5\""));
        assert!(q.contains("href=\"ch02a-solutions.html#s5\""));
        assert!(s.contains("id=\"s5\""));
        assert!(q.contains("[B, Section 3]"));
    }

    #[test]
    fn resolves_terms_equations_and_figures() {
        let src = "+++\nnumber = 2\nlabel = \"2A\"\ntitle = \"T\"\nquestion = \"Q?\"\n+++\nThe {{Extraction yield (EY)|extraction yield}} obeys {eq:ey}; see {fig:chart}. Again {{extraction yield}}.\n\n~~~eq ey\nformula = \"E = S m<sub>b</sub>/m<sub>c</sub>\"\nwhere = [[\"E\", \"extraction yield\"]]\n~~~\n\n~~~figure\nid = \"chart\"\nfile = \"a.gif\"\nalt = \"a\"\ncaption = \"c\"\n~~~\n\n~~~keyterms\n~~~\n";
        let c = crate::parse::parse_chapter("ch02a", crate::domain::Part::Main, src).unwrap();
        let r = chapter_body(&c).unwrap();
        assert!(r.body.contains("<dfn id=\"t-extraction-yield\">extraction yield</dfn>"));
        assert_eq!(r.body.matches("id=\"t-extraction-yield\"").count(), 1);
        assert!(r.body.contains("<a href=\"#eq-ey\">Equation 2A.1</a>"));
        assert!(r.body.contains("<a href=\"#fig-chart\">Figure 2A.1</a>"));
        assert!(r.body.contains("id=\"eq-ey\">(2A.1)"));
        assert!(r.body.contains("<h2 id=\"key-terms\">Key terms</h2>"));
        assert_eq!(r.terms.len(), 1);
        let bad = src.replace("{fig:chart}", "{fig:nope}");
        let c = crate::parse::parse_chapter("ch02a", crate::domain::Part::Main, &bad).unwrap();
        assert!(chapter_body(&c).is_err());
    }

    #[test]
    fn index_forms() {
        let t = "In practice the Maillard reaction runs fast. Drift is slow. Again drift.";
        assert_eq!(index_form("Beer-Lambert law", t), "Beer-Lambert law");
        assert_eq!(index_form("Fick's first law", t), "Fick's first law");
        assert_eq!(index_form("Extraction yield (EY)", t), "extraction yield (EY)");
        assert_eq!(index_form("Le Chatelier's principle", t), "Le Chatelier's principle");
        assert_eq!(index_form("Control", "See the Brewing Control Chart."), "control");
        assert_eq!(index_form("Griffith criterion", "In 1921 A. A. Griffith showed it."), "Griffith criterion");
        assert_eq!(index_form("Maillard reaction", t), "Maillard reaction");
        assert_eq!(index_form("Drift", t), "drift");
        assert_eq!(index_form("AEDA", t), "AEDA");
        assert_eq!(index_form("2-Furfurylthiol", t), "2-Furfurylthiol");
    }

    #[test]
    fn example_steps_are_labelled() {
        assert!(example("T", "Problem. Find x.\n\nCheck. Fine.").contains("<span class=\"step\">Problem.</span>"));
    }
}

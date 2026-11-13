//! IO boundary: read a content directory into a `Book`, write a site.
//! Everything between load and write is pure (`build_pages`).

use crate::domain::{Block, Book, BookMeta, Chapter, Part, Term};
use crate::lint::{self, Finding};
use crate::parse;
use crate::render::{self, Nav};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const CSS: &str = include_str!("../assets/book.css");
pub const JS: &str = include_str!("../assets/book.js");
pub const DARK: &str = include_str!("../assets/dark.css");

/// Size budgets in bytes.
pub const PAGE_BUDGET: usize = 96 * 1024;
pub const PRINT_BUDGET: usize = 1536 * 1024;
pub const SITE_BUDGET: usize = 3 * 1024 * 1024;
pub const FIGURE_BUDGET: usize = 25 * 1024;

#[derive(serde::Deserialize)]
struct GlossaryFile {
    #[serde(rename = "term")]
    terms: Vec<Term>,
}

fn read(p: &Path) -> Result<String, String> {
    fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
}

fn md_files(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .collect();
    paths.sort();
    Ok(paths)
}

fn load_dir(dir: &Path, part: Part) -> Result<Vec<Chapter>, String> {
    md_files(dir)?
        .iter()
        .map(|p| {
            let slug = p.file_stem().unwrap().to_string_lossy().to_string();
            parse::parse_chapter(&slug, part, &read(p)?).map_err(|e| format!("{}: {e}", p.display()))
        })
        .collect()
}

pub fn load(content: &Path) -> Result<Book, String> {
    let meta: BookMeta = toml::from_str(&read(&content.join("book.toml"))?).map_err(|e| format!("book.toml: {e}"))?;
    let glossary: GlossaryFile =
        toml::from_str(&read(&content.join("glossary.toml"))?).map_err(|e| format!("glossary.toml: {e}"))?;
    Ok(Book {
        meta,
        front: load_dir(&content.join("front"), Part::Front)?,
        chapters: load_dir(&content.join("chapters"), Part::Main)?,
        appendices: load_dir(&content.join("appendices"), Part::Appendix)?,
        glossary: glossary.terms,
    })
}

/// Figure file names used by the book.
pub fn figures(book: &Book) -> Vec<String> {
    book.all()
        .flat_map(|c| c.blocks.iter())
        .filter_map(|b| if let Block::Figure(f) = b { Some(f.file.clone()) } else { None })
        .collect()
}

/// All running text of the book, for questions such as "is this word capitalised mid-sentence?".
fn prose(book: &Book) -> String {
    book.all()
        .flat_map(|c| &c.blocks)
        .filter_map(|b| match b {
            // Reference lists hold titles, which capitalise ordinary words.
            Block::Prose(s) => s.split("## References").next().and_then(|s| s.split("## Further reading").next()),
            Block::Evidence(s) | Block::Misconception(s) => Some(s.as_str()),
            Block::Example { body, .. } | Block::Deeper { body, .. } => Some(body.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Short name of a chapter for cross-references ("Chapter 2A", "Appendix C").
pub fn place(c: &Chapter) -> String {
    match c.part {
        Part::Front => c.meta.title.clone(),
        Part::Main => format!("Chapter {}", c.meta.label()),
        Part::Appendix => format!("Appendix {}", c.meta.label()),
    }
}

/// Every place a term is defined: key -> (display text, [(link, place)]).
pub type TermIndex = std::collections::BTreeMap<String, (String, Vec<(String, String)>)>;

/// Render every chapter once; map each defined term to its first definition,
/// and collect every definition for the index of terms.
pub fn render_all(book: &Book) -> Result<(Vec<render::Rendered>, HashMap<String, (String, String)>, TermIndex), String> {
    let mut defs = HashMap::new();
    let mut index = TermIndex::new();
    let mut rs = Vec::new();
    let corpus = prose(book);
    for c in book.all() {
        let mut r = render::chapter_body(c)?;
        r.terms.iter_mut().for_each(|t| t.text = render::index_form(&t.text, &corpus));
        for t in &r.terms {
            let at = (format!("{}.html#{}", c.slug, t.anchor), place(c));
            defs.entry(t.key.clone()).or_insert_with(|| at.clone());
            index.entry(t.key.clone()).or_insert_with(|| (t.text.clone(), Vec::new())).1.push(at);
        }
        rs.push(r);
    }
    Ok((rs, defs, index))
}

/// The index of defined terms, grouped by initial letter.
pub fn term_index(index: &TermIndex) -> String {
    let mut h = String::from("<p>Every term defined in the text, with links to where it is defined. Terms with a short definition are also in the <a href=\"glossary.html\">glossary</a>.</p>\n");
    let mut letter = None;
    for (key, (text, places)) in index {
        let first = key.chars().find(|c| c.is_alphabetic()).map(|c| c.to_ascii_uppercase());
        if first != letter {
            if letter.is_some() {
                h.push_str("</ul>\n");
            }
            h.push_str(&format!("<h2>{}</h2>\n<ul class=\"tindex\">\n", first.unwrap_or('#')));
            letter = first;
        }
        let links: Vec<String> = places.iter().map(|(href, p)| format!("<a href=\"{href}\">{}</a>", render::esc(p))).collect();
        h.push_str(&format!("<li>{text}: {}</li>\n", links.join(", ")));
    }
    if letter.is_some() {
        h.push_str("</ul>\n");
    }
    h
}

/// Everything a build produces before it is written: the web pages, plus the
/// rendered pieces the other editions reuse.
pub struct Built {
    /// (file name, contents) for every text file of the web book.
    pub pages: Vec<(String, String)>,
    pub rendered: Vec<render::Rendered>,
    pub index: TermIndex,
    /// Glossary page body.
    pub glossary: String,
    /// Normalised glossary key -> definition as HTML.
    pub definitions: HashMap<String, String>,
    pub contents: String,
    pub terms: String,
    pub license: String,
    pub changes: String,
}

/// Width and height of a GIF or PNG from its header.
pub fn image_size(b: &[u8]) -> Option<(u32, u32)> {
    if b.len() >= 10 && b.starts_with(b"GIF") {
        return Some((u16::from_le_bytes([b[6], b[7]]) as u32, u16::from_le_bytes([b[8], b[9]]) as u32));
    }
    if b.len() >= 24 && b.starts_with(&[0x89, b'P', b'N', b'G']) {
        let be = |i: usize| u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        return Some((be(16), be(20)));
    }
    None
}

/// Give every figure its width and height so the browser lays the page out
/// once, before the images arrive, instead of again as each one loads.
pub fn size_images(html: &str, dims: &HashMap<String, (u32, u32)>) -> String {
    let mut out = html.to_string();
    for (f, (w, h)) in dims {
        let at = format!("<img src=\"figures/{f}\"");
        out = out.replace(&at, &format!("{at} width=\"{w}\" height=\"{h}\""));
    }
    out
}

/// Read the size of every figure the book uses; missing files are skipped (the check reports them).
pub fn figure_sizes(content: &Path, book: &Book) -> HashMap<String, (u32, u32)> {
    figures(book)
        .into_iter()
        .filter_map(|f| {
            let size = fs::read(content.join("figures").join(&f)).ok().as_deref().and_then(image_size)?;
            Some((f, size))
        })
        .collect()
}

/// Build every page of the web book.
pub fn build_pages(book: &Book, license_md: &str, changes_md: &str, dims: &HashMap<String, (u32, u32)>) -> Result<Built, String> {
    let m = &book.meta;
    let none = Nav { prev: None, next: None };
    let (rendered, defs, index) = render_all(book)?;
    let gloss = render::glossary(&book.glossary, &defs);
    let contents = render::contents(book);
    let terms = term_index(&index);
    let license = render::markdown(license_md);
    let changes = render::markdown(changes_md);
    let mut out = vec![
        ("book.css".to_string(), CSS.to_string()),
        ("dark.css".to_string(), DARK.to_string()),
        ("book.js".to_string(), JS.to_string()),
        ("index.html".to_string(), render::page(m, &m.title, None, &contents, &none)),
        ("glossary.html".to_string(), render::page(m, "Glossary", None, &gloss, &none)),
        ("terms.html".to_string(), render::page(m, "Index of terms", None, &terms, &none)),
        ("license.html".to_string(), render::page(m, "Licence", None, &license, &none)),
        ("changes.html".to_string(), render::page(m, "Changes and errata", None, &changes, &none)),
    ];

    let all: Vec<&Chapter> = book.all().collect();
    let titles: Vec<(String, String)> =
        all.iter().map(|c| (format!("{}.html", c.slug), render::heading(c))).collect();
    let mut print = String::new();
    for (i, c) in all.iter().enumerate() {
        let r = &rendered[i];
        let nav = Nav {
            prev: i.checked_sub(1).map(|j| (titles[j].0.as_str(), titles[j].1.as_str())),
            next: titles.get(i + 1).map(|t| (t.0.as_str(), t.1.as_str())),
        };
        let heading = &titles[i].1;
        let mut body = r.body.clone();
        if !r.solutions.is_empty() {
            body.push_str(&format!(
                "<p class=\"small\">Worked solutions to all problems: <a href=\"{}\">{} solutions</a>.</p>\n",
                render::solutions_file(&c.slug),
                render::esc(heading)
            ));
            let back = Nav { prev: Some((titles[i].0.as_str(), heading.as_str())), next: None };
            out.push((
                render::solutions_file(&c.slug),
                render::page(m, &format!("Solutions: {heading}"), None, &r.solutions, &back),
            ));
        }
        out.push((titles[i].0.clone(), render::page(m, heading, Some(&c.meta.question), &body, &nav)));
        let prefix = format!("p{i}-");
        print.push_str(&format!(
            "<h1 class=\"pb\">{}</h1>\n<p class=\"lead\">{}</p>\n{}",
            render::esc(heading),
            render::esc(&c.meta.question),
            // Ids must stay unique on the combined page; solution links go to the solutions files.
            r.body
                .replace("id=\"", &format!("id=\"{prefix}"))
                .replace("for=\"", &format!("for=\"{prefix}"))
                .replace("href=\"#", &format!("href=\"#{prefix}"))
        ));
        if !r.solutions.is_empty() {
            print.push_str(&format!(
                "<h2>Solutions</h2>\n{}",
                r.solutions.replace("id=\"", &format!("id=\"{prefix}"))
            ));
        }
    }
    print.push_str(&format!("<h1 class=\"pb\">Glossary</h1>\n{gloss}"));
    out.push(("print.html".to_string(), render::page(m, &format!("{} (complete)", m.title), None, &print, &none)));
    for page in out.iter_mut().filter(|p| p.0.ends_with(".html")) {
        page.1 = size_images(&page.1, dims);
    }
    let definitions = book.glossary.iter().map(|t| (render::term_key(&t.term), render::inline_md(&t.definition))).collect();
    Ok(Built { pages: out, rendered, index, glossary: gloss, definitions, contents, terms, license, changes })
}


pub struct Report {
    pub prose: Vec<(String, Finding)>,
    pub citations: Vec<(String, Finding)>,
    pub compat: Vec<(String, Finding)>,
    pub glyphs: Vec<(String, Finding)>,
    pub figures: Vec<(String, Finding)>,
    pub terms: Vec<(String, Finding)>,
    pub sizes: Vec<(String, usize)>,
}

pub fn check(content: &Path, book: &Book, pages: &[(String, String)]) -> Result<Report, String> {
    let (mut prose, mut citations) = (Vec::new(), Vec::new());
    for sub in ["front", "chapters", "appendices"] {
        for p in md_files(&content.join(sub))? {
            let name = p.file_name().unwrap().to_string_lossy().to_string();
            let src = read(&p)?;
            prose.extend(lint::prose(&src).into_iter().map(|f| (name.clone(), f)));
            citations.extend(lint::citations(&src).into_iter().map(|f| (name.clone(), f)));
        }
    }
    let mut figs = Vec::new();
    for f in figures(book) {
        let path = content.join("figures").join(&f);
        match fs::read(&path) {
            Ok(bytes) => figs.extend(lint::figure(&bytes, FIGURE_BUDGET).into_iter().map(|x| (f.clone(), x))),
            Err(_) => figs.push((f.clone(), Finding { rule: "figure missing", line: 0, excerpt: path.display().to_string() })),
        }
    }
    let (_, defs, _) = render_all(book)?;
    let terms = book
        .glossary
        .iter()
        .filter(|t| !defs.contains_key(&render::term_key(&t.term)))
        .map(|t| ("glossary.toml".to_string(), Finding { rule: "glossary term never defined in text", line: 0, excerpt: t.term.clone() }))
        .collect();
    let (mut compat, mut glyphs, mut sizes) = (Vec::new(), Vec::new(), Vec::new());
    for (name, body) in pages {
        sizes.push((name.clone(), body.len()));
        if name.ends_with(".html") {
            compat.extend(lint::compat(body).into_iter().map(|f| (name.clone(), f)));
            glyphs.extend(lint::glyphs(body).into_iter().map(|f| (name.clone(), f)));
        }
    }
    Ok(Report { prose, citations, compat, glyphs, figures: figs, terms, sizes })
}

pub fn write(content: &Path, out: &Path, book: &Book, pages: &[(String, String)], extra: &[(&str, &str)]) -> Result<usize, String> {
    fs::create_dir_all(out.join("figures")).map_err(|e| e.to_string())?;
    for (name, body) in pages {
        fs::write(out.join(name), body).map_err(|e| format!("{name}: {e}"))?;
    }
    for (name, body) in extra {
        // Plain-text files use CRLF so that Notepad on old Windows shows line breaks.
        fs::write(out.join(name), body.replace('\n', "\r\n")).map_err(|e| format!("{name}: {e}"))?;
    }
    let mut bytes = 0;
    for f in figures(book) {
        let src = content.join("figures").join(&f);
        if let Ok(n) = fs::copy(&src, out.join("figures").join(&f)) {
            bytes += n as usize;
        }
    }
    Ok(bytes)
}

/// Write (path, bytes) pairs to a zip file. A file named `mimetype` is stored
/// uncompressed, as the EPUB container requires of its first entry.
pub fn write_zip(path: &Path, files: &[(String, Vec<u8>)]) -> Result<(), String> {
    use std::io::Write;
    use zip::write::SimpleFileOptions;
    let err = |e: &dyn std::fmt::Display| format!("{}: {e}", path.display());
    let file = fs::File::create(path).map_err(|e| err(&e))?;
    let mut z = zip::ZipWriter::new(file);
    // Fixed timestamps keep the archive identical from one build to the next.
    let base = SimpleFileOptions::default().last_modified_time(zip::DateTime::default());
    for (name, bytes) in files {
        let opts = if name == "mimetype" {
            base.compression_method(zip::CompressionMethod::Stored)
        } else {
            base.compression_method(zip::CompressionMethod::Deflated).compression_level(Some(9))
        };
        z.start_file(name.as_str(), opts).map_err(|e| err(&e))?;
        z.write_all(bytes).map_err(|e| err(&e))?;
    }
    z.finish().map_err(|e| err(&e))?;
    Ok(())
}

/// Every file under `dir` (one level of subfolders), named `prefix/relative/path`.
/// Hidden files are left out.
pub fn folder_files(dir: &Path, prefix: &str) -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut out = Vec::new();
    let mut walk = vec![(dir.to_path_buf(), prefix.to_string())];
    while let Some((d, p)) = walk.pop() {
        let mut entries: Vec<_> = fs::read_dir(&d).map_err(|e| format!("{}: {e}", d.display()))?.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.file_name());
        for e in entries {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            let path = e.path();
            if path.is_dir() {
                walk.push((path, format!("{p}/{name}")));
            } else {
                out.push((format!("{p}/{name}"), fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_sizes_and_attributes() {
        let gif = [b'G', b'I', b'F', b'8', b'7', b'a', 0xcb, 0x01, 0x0d, 0x01];
        assert_eq!(image_size(&gif), Some((459, 269)));
        assert_eq!(image_size(b"nope"), None);
        let dims: HashMap<String, (u32, u32)> = [("a.gif".to_string(), (10, 20))].into();
        assert_eq!(size_images("<img src=\"figures/a.gif\" alt=\"x\">", &dims), "<img src=\"figures/a.gif\" width=\"10\" height=\"20\" alt=\"x\">");
    }
}

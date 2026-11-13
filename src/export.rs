//! Other editions built from the rendered web pages: a static (script-free)
//! form of each page, an EPUB, a single HTML document for PDF typesetting,
//! and a flashcard file for Anki. Everything here is pure: it takes strings
//! and returns strings or bytes; `site.rs` does the file writing.

use crate::domain::{Block, Book, Chapter, Part, Question};
use crate::render::{self, DefinedTerm, Rendered};
use std::collections::HashMap;

// ---------------------------------------------------------------- static pages

/// Turn interactive forms into printed text: inputs show their starting
/// values, radio buttons become circles, buttons and script-only output go.
/// Used for the EPUB and PDF editions, where readers cannot run calculators.
pub fn static_html(h: &str) -> String {
    let mut out = String::with_capacity(h.len());
    let mut i = 0;
    while i < h.len() {
        let rest = &h[i..];
        if !rest.starts_with('<') {
            let next = rest.find('<').unwrap_or(rest.len());
            out.push_str(&rest[..next]);
            i += next;
            continue;
        }
        let Some(close) = rest.find('>') else {
            out.push_str(rest);
            break;
        };
        let tag = &rest[..=close];
        let name = tag_name(tag);
        let skip_to = |end: &str| rest.find(end).map(|e| e + end.len()).unwrap_or(rest.len());
        let advance = match name.as_str() {
            "form" => {
                out.push_str("<div class=\"form\">");
                close + 1
            }
            "/form" => {
                out.push_str("</div>");
                close + 1
            }
            "input" => {
                match attr(tag, "type").as_deref() {
                    Some("text") => match attr(tag, "value") {
                        Some(v) => out.push_str(&format!("<span class=\"val\">{v}</span>")),
                        None => out.push_str("<span class=\"blank\">__________</span>"),
                    },
                    Some("radio") => out.push_str("&#9675;"),
                    _ => {}
                }
                close + 1
            }
            "select" => {
                let end = skip_to("</select>");
                let inner = &rest[..end];
                let chosen = inner.find("selected").or_else(|| inner.find("<option"));
                let text = chosen
                    .and_then(|p| inner[p..].find('>').map(|q| p + q + 1))
                    .map(|s| &inner[s..s + inner[s..].find('<').unwrap_or(0)])
                    .unwrap_or("");
                out.push_str(&format!("<span class=\"val\">{text}</span>"));
                end
            }
            "textarea" => {
                let end = skip_to("</textarea>");
                let body = &rest[close + 1..end - "</textarea>".len()];
                out.push_str(&format!("<pre class=\"val\">{body}</pre>"));
                end
            }
            "label" => {
                out.push_str("<span>");
                close + 1
            }
            "/label" => {
                out.push_str("</span>");
                close + 1
            }
            "noscript" => skip_to("</noscript>"),
            "script" => skip_to("</script>"),
            "button" => skip_to("</button>"),
            "table" if attr(tag, "class").as_deref() == Some("bcc") => skip_to("</table>"),
            "div" if attr(tag, "id").is_some_and(|v| v.ends_with("-out")) => skip_to("</div>"),
            "p" if attr(tag, "class").as_deref() == Some("fb") => skip_to("</p>"),
            _ => {
                out.push_str(tag);
                close + 1
            }
        };
        i += advance;
    }
    // Hidden fields and buttons leave empty containers behind.
    let out = out.replace("<div></div>\n", "").replace("<p></p>\n", "").replace("<p></p>", "");
    // Say once per calculator that it is interactive only on the web.
    out.replace(
        "<div class=\"widget\">\n<h3>",
        "<div class=\"widget\">\n<p class=\"small\">Calculator (interactive in the web edition). The values shown are its starting inputs; the formula below lets you work it by hand.</p>\n<h3>",
    )
}

fn tag_name(tag: &str) -> String {
    tag.trim_start_matches('<')
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '/')
        .collect::<String>()
        .to_ascii_lowercase()
}

/// Value of a double-quoted attribute.
fn attr(tag: &str, name: &str) -> Option<String> {
    let key = format!(" {name}=\"");
    let s = tag.find(&key)? + key.len();
    let e = tag[s..].find('"')?;
    Some(tag[s..s + e].to_string())
}

// ---------------------------------------------------------------- XHTML

/// Named entities used by the book, as code points. XHTML in EPUB knows only
/// the five XML entities, so every other entity becomes a numeric reference.
const ENTITIES: &[(&str, u32)] = &[
    ("nbsp", 160), ("copy", 169), ("deg", 176), ("plusmn", 177), ("micro", 181), ("middot", 183),
    ("times", 215), ("divide", 247), ("ndash", 8211), ("mdash", 8212), ("lsquo", 8216), ("rsquo", 8217),
    ("ldquo", 8220), ("rdquo", 8221), ("hellip", 8230), ("minus", 8722), ("radic", 8730), ("prop", 8733),
    ("infin", 8734), ("int", 8747), ("asymp", 8776), ("ne", 8800), ("le", 8804), ("ge", 8805),
    ("larr", 8592), ("rarr", 8594), ("harr", 8596), ("uarr", 8593), ("darr", 8595), ("part", 8706),
    ("sum", 8721), ("prime", 8242), ("auml", 228), ("ouml", 246), ("uuml", 252), ("eacute", 233),
    ("aelig", 230), ("oslash", 248), ("aring", 229),
    ("Alpha", 913), ("Beta", 914), ("Gamma", 915), ("Delta", 916), ("Theta", 920), ("Lambda", 923),
    ("Pi", 928), ("Sigma", 931), ("Phi", 934), ("Psi", 936), ("Omega", 937),
    ("alpha", 945), ("beta", 946), ("gamma", 947), ("delta", 948), ("epsilon", 949), ("zeta", 950),
    ("eta", 951), ("theta", 952), ("kappa", 954), ("lambda", 955), ("mu", 956), ("nu", 957), ("xi", 958),
    ("pi", 960), ("rho", 961), ("sigma", 963), ("tau", 964), ("phi", 966), ("chi", 967), ("psi", 968),
    ("omega", 969),
];

/// Convert the book's HTML to well-formed XHTML: self-close void elements,
/// replace named entities, and drop attributes that HTML5 made obsolete.
pub fn xhtml(h: &str) -> Result<String, String> {
    let ents: HashMap<&str, u32> = ENTITIES.iter().copied().collect();
    let mut out = String::with_capacity(h.len() + h.len() / 20);
    let mut i = 0;
    while i < h.len() {
        let rest = &h[i..];
        let c = rest.chars().next().unwrap();
        if c == '&' {
            if let Some(semi) = rest[1..].find(';').filter(|&n| n < 12) {
                let name = &rest[1..1 + semi];
                if name.starts_with('#') || matches!(name, "amp" | "lt" | "gt" | "quot" | "apos") {
                    out.push_str(&rest[..semi + 2]);
                } else if let Some(cp) = ents.get(name) {
                    out.push_str(&format!("&#{cp};"));
                } else if name.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err(format!("entity &{name}; has no numeric mapping in export::ENTITIES"));
                } else {
                    out.push_str("&amp;");
                    i += 1;
                    continue;
                }
                i += semi + 2;
                continue;
            }
            out.push_str("&amp;");
            i += 1;
            continue;
        }
        if c == '<' && !rest.starts_with("<!--") {
            if let Some(close) = rest.find('>') {
                let mut tag = rest[..close].to_string();
                let name = tag_name(&tag);
                if let Some(p) = tag.find(" summary=\"") {
                    let e = tag[p + 10..].find('"').map(|e| p + 10 + e + 1).unwrap_or(tag.len());
                    tag.replace_range(p..e, "");
                }
                let void = matches!(name.as_str(), "br" | "hr" | "img" | "input" | "meta" | "link" | "col");
                if void && !tag.ends_with('/') {
                    tag.push_str(" /");
                }
                out.push_str(&tag);
                out.push('>');
                i += close + 1;
                continue;
            }
        }
        out.push(c);
        i += c.len_utf8();
    }
    Ok(out)
}

// ---------------------------------------------------------------- links between files

/// Rewrites links for an edition. `map` takes a web file name (without
/// directory) to its new target prefix: a file name for EPUB, or an id prefix
/// for the single-document PDF.
fn rewrite_links(h: &str, rewrite: &dyn Fn(&str) -> String) -> String {
    let mut out = String::with_capacity(h.len());
    let mut rest = h;
    while let Some(p) = rest.find("href=\"") {
        out.push_str(&rest[..p + 6]);
        rest = &rest[p + 6..];
        let e = rest.find('"').unwrap_or(rest.len());
        out.push_str(&rewrite(&rest[..e]));
        rest = &rest[e..];
    }
    out.push_str(rest);
    out
}

// ---------------------------------------------------------------- EPUB

pub struct EpubInput<'a> {
    pub book: &'a Book,
    pub rendered: &'a [Rendered],
    pub contents: &'a str,
    pub glossary: &'a str,
    pub terms: &'a str,
    pub license: &'a str,
    pub changes: &'a str,
    pub css: &'a str,
    /// ISO 8601 UTC timestamp for dcterms:modified.
    pub modified: &'a str,
}

fn xpage(lang: &str, title: &str, lead: Option<&str>, body: &str, foot: &str) -> String {
    let lead = lead.map(|l| format!("<p class=\"lead\">{}</p>\n", render::esc(l))).unwrap_or_default();
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE html>\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\" lang=\"{lang}\" xml:lang=\"{lang}\">\n<head>\n<meta charset=\"utf-8\"/>\n<title>{t}</title>\n<link rel=\"stylesheet\" type=\"text/css\" href=\"book.css\"/>\n</head>\n<body>\n<div class=\"page\">\n<h1>{t}</h1>\n{lead}{body}\n<p class=\"foot\">{foot}</p>\n</div>\n</body>\n</html>\n",
        t = render::esc(title)
    )
}

fn epub_link(target: &str) -> String {
    if target.starts_with('#') || target.starts_with("http") || target.starts_with("mailto:") {
        return target.to_string();
    }
    let (file, frag) = target.split_once('#').map(|(f, x)| (f, Some(x))).unwrap_or((target, None));
    let file = file.strip_suffix(".html").map(|f| format!("{f}.xhtml")).unwrap_or_else(|| file.to_string());
    match frag {
        Some(x) => format!("{file}#{x}"),
        None => file,
    }
}

/// Deterministic UUID-shaped identifier from the title and version (FNV-1a).
fn book_uuid(seed: &str) -> String {
    let mut a: u64 = 0xcbf29ce484222325;
    let mut b: u64 = 0x84222325cbf29ce4;
    for byte in seed.bytes() {
        a = (a ^ byte as u64).wrapping_mul(0x100000001b3);
        b = (b ^ byte as u64).wrapping_mul(0x100000001b3).rotate_left(7);
    }
    let h = format!("{a:016x}{b:016x}");
    format!("{}-{}-5{}-a{}-{}", &h[0..8], &h[8..12], &h[13..16], &h[17..20], &h[20..32])
}

/// Every file of the EPUB as (path, contents), mimetype first.
pub fn epub(input: &EpubInput) -> Result<Vec<(String, Vec<u8>)>, String> {
    let book = input.book;
    let m = &book.meta;
    let lang = &m.lang;
    let foot = xhtml(&render::footer(m, ".xhtml"))?;
    let mut files: Vec<(String, Vec<u8>)> = vec![
        ("mimetype".into(), b"application/epub+zip".to_vec()),
        (
            "META-INF/container.xml".into(),
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<container version=\"1.0\" xmlns=\"urn:oasis:names:tc:opendocument:xmlns:container\">\n<rootfiles><rootfile full-path=\"OEBPS/content.opf\" media-type=\"application/oebps-package+xml\"/></rootfiles>\n</container>\n".to_vec(),
        ),
    ];
    // (file, title, in table of contents, linear)
    let mut spine: Vec<(String, String, bool, bool)> = Vec::new();
    let mut add = |files: &mut Vec<(String, Vec<u8>)>, name: &str, title: &str, lead: Option<&str>, body: &str, toc: bool, linear: bool| -> Result<(), String> {
        let body = xhtml(&rewrite_links(&static_html(body), &epub_link))?;
        files.push((format!("OEBPS/{name}"), xpage(lang, title, lead, &body, &foot).into_bytes()));
        spine.push((name.to_string(), title.to_string(), toc, linear));
        Ok(())
    };
    let contents = input
        .contents
        .lines()
        .filter(|l| !l.contains("print.html") && !l.contains("flashcards"))
        .collect::<Vec<_>>()
        .join("\n");
    add(&mut files, "index.xhtml", &m.title, None, &contents, true, true)?;
    let all: Vec<&Chapter> = book.all().collect();
    let mut solutions = Vec::new();
    for (c, r) in all.iter().zip(input.rendered) {
        let title = render::heading(c);
        add(&mut files, &format!("{}.xhtml", c.slug), &title, Some(&c.meta.question), &r.body, true, true)?;
        if !r.solutions.is_empty() {
            solutions.push((c, r));
        }
    }
    for (c, r) in solutions {
        let back = format!("<p><a href=\"{}.html\">Back to {}</a></p>\n", c.slug, render::esc(&render::heading(c)));
        add(&mut files, &format!("{}-solutions.xhtml", c.slug), &format!("Solutions: {}", render::heading(c)), None, &format!("{back}{}", r.solutions), false, false)?;
    }
    add(&mut files, "glossary.xhtml", "Glossary", None, input.glossary, true, true)?;
    add(&mut files, "terms.xhtml", "Index of terms", None, input.terms, true, true)?;
    add(&mut files, "changes.xhtml", "Changes and errata", None, input.changes, true, true)?;
    add(&mut files, "license.xhtml", "Licence", None, input.license, true, true)?;

    let nav_items: String = spine
        .iter()
        .filter(|s| s.2)
        .map(|(f, t, _, _)| format!("<li><a href=\"{f}\">{}</a></li>", render::esc(t)))
        .collect::<Vec<_>>()
        .join("\n");
    let nav = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE html>\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\" lang=\"{lang}\" xml:lang=\"{lang}\">\n<head><meta charset=\"utf-8\"/><title>Contents</title></head>\n<body>\n<nav epub:type=\"toc\" id=\"toc\"><h1>Contents</h1>\n<ol>\n{nav_items}\n</ol></nav>\n<nav epub:type=\"landmarks\" hidden=\"hidden\"><ol>\n<li><a epub:type=\"toc\" href=\"index.xhtml\">Contents</a></li>\n<li><a epub:type=\"bodymatter\" href=\"{first}\">Start</a></li>\n</ol></nav>\n</body>\n</html>\n",
        first = spine.iter().find(|s| s.0.starts_with("ch")).map(|s| s.0.as_str()).unwrap_or("index.xhtml")
    );
    files.push(("OEBPS/nav.xhtml".into(), nav.into_bytes()));

    let uid = book_uuid(&format!("{}|{}|{}", m.title, m.edition, m.version.as_deref().unwrap_or("")));
    let ncx_points: String = spine
        .iter()
        .filter(|s| s.2)
        .enumerate()
        .map(|(i, (f, t, _, _))| format!("<navPoint id=\"n{}\" playOrder=\"{}\"><navLabel><text>{}</text></navLabel><content src=\"{f}\"/></navPoint>", i + 1, i + 1, render::esc(t)))
        .collect::<Vec<_>>()
        .join("\n");
    let ncx = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ncx xmlns=\"http://www.daisy.org/z3986/2005/ncx/\" version=\"2005-1\">\n<head><meta name=\"dtb:uid\" content=\"urn:uuid:{uid}\"/></head>\n<docTitle><text>{}</text></docTitle>\n<navMap>\n{ncx_points}\n</navMap>\n</ncx>\n",
        render::esc(&m.title)
    );
    files.push(("OEBPS/toc.ncx".into(), ncx.into_bytes()));
    files.push(("OEBPS/book.css".into(), format!("{}\n.page {{ max-width: none; margin: 0; padding: 0 0.5em; }}\n.val {{ font-family: \"Courier New\", monospace; }}\n", input.css).into_bytes()));

    let mut manifest = String::from(
        "<item id=\"nav\" href=\"nav.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\"/>\n<item id=\"ncx\" href=\"toc.ncx\" media-type=\"application/x-dtbncx+xml\"/>\n<item id=\"css\" href=\"book.css\" media-type=\"text/css\"/>\n",
    );
    for (i, (f, _, _, _)) in spine.iter().enumerate() {
        manifest.push_str(&format!("<item id=\"x{i}\" href=\"{f}\" media-type=\"application/xhtml+xml\"/>\n"));
    }
    for (i, f) in crate::site::figures(book).iter().enumerate() {
        let mt = if f.ends_with(".png") { "image/png" } else { "image/gif" };
        manifest.push_str(&format!("<item id=\"f{i}\" href=\"figures/{f}\" media-type=\"{mt}\"/>\n"));
    }
    let itemrefs: String = spine
        .iter()
        .enumerate()
        .map(|(i, s)| format!("<itemref idref=\"x{i}\"{}/>", if s.3 { "" } else { " linear=\"no\"" }))
        .collect::<Vec<_>>()
        .join("\n");
    let opf = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\" unique-identifier=\"uid\" xml:lang=\"{lang}\">\n<metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\">\n<dc:identifier id=\"uid\">urn:uuid:{uid}</dc:identifier>\n<dc:title>{title}</dc:title>\n<dc:creator>{author}</dc:creator>\n<dc:language>{lang}</dc:language>\n<dc:publisher>{publisher}</dc:publisher>\n<dc:rights>{rights}</dc:rights>\n<meta property=\"dcterms:modified\">{modified}</meta>\n</metadata>\n<manifest>\n{manifest}</manifest>\n<spine toc=\"ncx\">\n{itemrefs}\n</spine>\n</package>\n",
        title = render::esc(&m.title),
        author = render::esc(&m.author),
        publisher = render::esc(m.publisher.as_deref().unwrap_or("")),
        rights = render::esc(&match &m.buyer {
            Some(b) => format!("Copyright {} {}. Licensed to {b} for personal use.", m.year, m.license_holder),
            None => format!("Copyright {} {}. Licensed for personal use.", m.year, m.license_holder),
        }),
        modified = input.modified,
    );
    files.push(("OEBPS/content.opf".into(), opf.into_bytes()));
    Ok(files)
}

// ---------------------------------------------------------------- PDF (one HTML document for a paged-media typesetter)

/// One HTML document holding the whole book, with ids made unique per part and
/// links rewritten to point inside the document, plus paged-media CSS for
/// running heads, page numbers, and page references in the contents and index.
pub fn pdf_document(book: &Book, rendered: &[Rendered], glossary: &str, index: &crate::site::TermIndex, css: &str) -> String {
    let m = &book.meta;
    let all: Vec<&Chapter> = book.all().collect();
    let prefix: HashMap<&str, usize> = all.iter().enumerate().map(|(i, c)| (c.slug.as_str(), i)).collect();
    let link = |own: usize, own_sol: bool| {
        let prefix = prefix.clone();
        move |t: &str| -> String {
            if t.starts_with("http") || t.starts_with("mailto:") {
                return t.to_string();
            }
            let (file, frag) = t.split_once('#').map(|(f, x)| (f, Some(x))).unwrap_or((t, None));
            if file.is_empty() {
                let p = if own_sol { format!("s{own}-") } else { format!("p{own}-") };
                return format!("#{p}{}", frag.unwrap_or(""));
            }
            let stem = file.trim_end_matches(".html");
            if let Some(base) = stem.strip_suffix("-solutions") {
                if let Some(i) = prefix.get(base) {
                    return format!("#s{i}-{}", frag.unwrap_or("top"));
                }
            }
            if let Some(i) = prefix.get(stem) {
                return format!("#p{i}-{}", frag.unwrap_or("top"));
            }
            match stem {
                "glossary" => format!("#g-{}", frag.unwrap_or("top")),
                "terms" => "#index".to_string(),
                _ => "#top".to_string(),
            }
        }
    };
    let ids = |h: &str, p: &str| h.replace(" id=\"", &format!(" id=\"{p}"));

    let mut toc = String::from("<ul class=\"ptoc\">\n");
    let mut body = String::new();
    for (i, (c, r)) in all.iter().zip(rendered).enumerate() {
        let title = render::heading(c);
        let cls = if c.part == Part::Front { "front" } else { "chap" };
        toc.push_str(&format!("<li><a href=\"#p{i}-top\">{}</a></li>\n", render::esc(&title)));
        let text = rewrite_links(&static_html(&r.body), &link(i, false));
        body.push_str(&format!(
            "<div class=\"{cls}\"><h1 class=\"ch\" id=\"p{i}-top\">{}</h1>\n<p class=\"lead\">{}</p>\n{}</div>\n",
            render::esc(&title),
            render::esc(&c.meta.question),
            ids(&text, &format!("p{i}-"))
        ));
    }
    let mut sols = String::new();
    for (i, (c, r)) in all.iter().zip(rendered).enumerate() {
        if r.solutions.is_empty() {
            continue;
        }
        let text = rewrite_links(&r.solutions, &link(i, true));
        sols.push_str(&format!("<h2 id=\"s{i}-top\">{}</h2>\n{}", render::esc(&render::heading(c)), ids(&text, &format!("s{i}-"))));
    }
    toc.push_str("<li><a href=\"#solutions\">Solutions to problems</a></li>\n<li><a href=\"#glossary\">Glossary</a></li>\n<li><a href=\"#index\">Index of terms</a></li>\n</ul>\n");

    let mut idx = String::from("<div class=\"pindex\">\n");
    for (_, (text, places)) in index {
        let refs: Vec<String> = places
            .iter()
            .map(|(href, _)| format!("<a class=\"pg\" href=\"{}\"></a>", link(0, false)(href)))
            .collect();
        idx.push_str(&format!("<p>{text} {}</p>\n", refs.join(", ")));
    }
    idx.push_str("</div>\n");
    let gloss = rewrite_links(glossary, &link(0, false));

    let licensee = m.buyer.as_deref().map(|b| format!("<p class=\"buyer\">Licensed to {} for personal use.</p>", render::esc(b))).unwrap_or_default();
    format!(
        "<!DOCTYPE html>\n<html lang=\"{lang}\">\n<head>\n<meta charset=\"utf-8\">\n<title>{title}</title>\n<style>\n{css}\n{PDF_CSS}\n</style>\n</head>\n<body>\n<div class=\"cover\" id=\"top\"><p class=\"ctitle\">{title}</p><p class=\"csub\">{sub}</p><p class=\"cauth\">{author}</p><p class=\"cfoot\">{edition} edition{version}, {year}</p>{licensee}</div>\n<div class=\"tocpage\"><h1 class=\"ch\">Contents</h1>\n{toc}</div>\n{body}<div class=\"chap\"><h1 class=\"ch\" id=\"solutions\">Solutions to problems</h1>\n{sols}</div>\n<div class=\"chap\"><h1 class=\"ch\" id=\"glossary\">Glossary</h1>\n{gloss}</div>\n<div class=\"chap\"><h1 class=\"ch\" id=\"index\">Index of terms</h1>\n{idx}</div>\n<p class=\"colophon\">{foot}</p>\n</body>\n</html>\n",
        lang = m.lang,
        title = render::esc(&m.title),
        sub = render::esc(m.subtitle.as_deref().unwrap_or("")),
        author = render::esc(&m.author),
        edition = render::esc(&m.edition),
        version = m.version.as_deref().map(|v| format!(", version {}", render::esc(v))).unwrap_or_default(),
        year = m.year,
        foot = render::footer(m, ".html").replace(" (<a href=\"changes.html\">changes and errata</a>)", "").replace("see <a href=\"license.html\">licence</a>", "see the licence page of the web edition"),
    )
}

const PDF_CSS: &str = r#"
@page { size: 7in 10in; margin: 0.85in 0.75in 0.9in 0.85in;
  @top-center { content: string(chap); font: 8pt "DejaVu Sans", sans-serif; color: #666; }
  @bottom-center { content: counter(page); font: 9pt "DejaVu Sans", sans-serif; } }
@page :first { @top-center { content: none; } @bottom-center { content: none; } }
html { font-size: 10pt; }
body { background: #fff; color: #000; font-family: "Bitstream Charter", "DejaVu Serif", Georgia, serif; line-height: 1.38; hyphens: auto; }
h1, h2, h3, .boxhead, .lbl, .step, .tier { font-family: "DejaVu Sans", Verdana, sans-serif; }
p, li { orphans: 2; widows: 2; }
h1, h2, h3 { break-after: avoid; }
h1.ch { string-set: chap content(); font-size: 18pt; margin-top: 0; }
.cover { page: cover; height: 8in; text-align: center; padding-top: 2.4in; break-after: page; }
.ctitle { font: bold 34pt "DejaVu Sans", sans-serif; margin: 0; }
.csub { font-size: 13pt; font-style: italic; margin: 0.6em 1in 2em 1in; }
.cauth { font-size: 13pt; }
.cfoot, .buyer { font-size: 9pt; color: #444; }
.tocpage, .chap, .front { break-before: page; }
.ptoc { list-style: none; padding: 0; }
.ptoc li { margin: 0.35em 0; }
.ptoc a { color: #000; text-decoration: none; }
.ptoc a::after { content: leader('.') target-counter(attr(href url), page); }
a { color: #000; text-decoration: none; }
a.pg::after { content: target-counter(attr(href url), page); }
.nav, .sol, .calc, .check, noscript { display: none; }
.quiz a[href*="ans-"]::after { content: " on page " target-counter(attr(href url), page); }
.widget, .quiz, .example, .evidence, .misc, .deeper, .figure, table.deq, .lab table { break-inside: avoid; }
.lab { break-before: auto; }
.figure img { max-width: 4.6in; }
.val { font-family: "DejaVu Sans Mono", monospace; font-size: 0.9em; border-bottom: 1px solid #999; }
.pindex p { margin: 0.1em 0; font-size: 9pt; }
.pindex { columns: 2; column-gap: 0.3in; }
dl.gloss dd { margin-left: 1em; }
.colophon { font-size: 8pt; color: #555; break-before: page; }
"#;

// ---------------------------------------------------------------- flashcards

fn clean(s: &str) -> String {
    s.replace(['\t', '\r'], " ").replace('\n', " ")
}

/// Anki-importable text: one card per defined term and per self-check question.
/// Columns: front, back, tags. HTML is allowed in both fields.
pub fn flashcards(book: &Book, rendered: &[Rendered], glossary: &HashMap<String, String>) -> String {
    let mut done = std::collections::HashSet::new();
    let mut out = String::from("#separator:tab\n#html:true\n#deck:Upstream\n#notetype:Basic\n#columns:Front\tBack\tTags\n#tags column:3\n");
    for (c, r) in book.all().zip(rendered) {
        let tag = format!("upstream {}", tag_for(c));
        // A term defined again in a later chapter gets one card, from its first definition.
        let mut seen: Vec<&DefinedTerm> = r.terms.iter().filter(|t| done.insert(t.key.clone())).collect();
        seen.sort_by(|a, b| a.key.cmp(&b.key));
        for t in seen {
            let back = glossary.get(&t.key).cloned().unwrap_or_else(|| t.context.clone());
            out.push_str(&format!("{}\t{}\t{tag} term\n", clean(&t.text), clean(&back)));
        }
        for b in &c.blocks {
            let Block::Quiz(q) = b else { continue };
            for question in &q.questions {
                let (front, answer) = match question {
                    Question::Choice { prompt, options, answer, .. } => {
                        let opts: Vec<String> = options
                            .iter()
                            .enumerate()
                            .map(|(i, o)| format!("({}) {}", (b'a' + i as u8) as char, render::inline_md(o)))
                            .collect();
                        (format!("{}<br>{}", render::inline_md(prompt), opts.join("<br>")), format!("({}) {}", (b'a' + *answer as u8) as char, render::inline_md(&options[*answer])))
                    }
                    Question::Numeric { prompt, answer, unit, .. } => (render::inline_md(prompt), format!("{answer} {unit}")),
                };
                out.push_str(&format!("{}\t<b>{}</b><br>{}\t{tag} check\n", clean(&front), clean(&answer), clean(&render::inline_md(question.explain()))));
            }
        }
    }
    out
}

fn tag_for(c: &Chapter) -> String {
    match c.part {
        Part::Front => "front".into(),
        Part::Main => format!("chapter-{}", c.meta.label()),
        Part::Appendix => format!("appendix-{}", c.meta.label()),
    }
}

// ---------------------------------------------------------------- dates

/// UTC timestamp "YYYY-MM-DDThh:mm:ssZ" from seconds since 1970 (civil-from-days).
pub fn iso8601(secs: u64) -> String {
    let days = (secs / 86400) as i64;
    let rem = secs % 86400;
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };
    format!("{y:04}-{mo:02}-{d:02}T{:02}:{:02}:{:02}Z", rem / 3600, rem % 3600 / 60, rem % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_forms_become_text() {
        let h = "<form class=\"w\" id=\"x\" action=\"#\"><div><input type=\"hidden\" name=\"kind\" value=\"k\"></div><label for=\"a\">Dose</label> <input type=\"text\" size=\"7\" name=\"a\" id=\"a\" value=\"15\"> g<select name=\"m\"><option value=\"p\">Perc</option><option value=\"i\" selected>Imm</option></select><input type=\"radio\" name=\"r\"> <label>A</label><p><input type=\"button\" class=\"calc\" value=\"Calculate\"></p><div class=\"out\" id=\"x-out\"></div></form><noscript>n</noscript>";
        let s = static_html(h);
        assert!(s.contains("<span class=\"val\">15</span>"));
        assert!(s.contains("<span class=\"val\">Imm</span>"));
        assert!(s.contains("&#9675;"));
        assert!(!s.contains("<input") && !s.contains("<form") && !s.contains("noscript") && !s.contains("x-out"));
    }

    #[test]
    fn xhtml_is_well_formed_enough() {
        let x = xhtml("<p>2&nbsp;&micro;m &amp; <br><img src=\"a.gif\" alt=\"a\"> AT&T</p><table summary=\"s\"><tr><td>1</td></tr></table>").unwrap();
        assert_eq!(x, "<p>2&#160;&#181;m &amp; <br /><img src=\"a.gif\" alt=\"a\" /> AT&amp;T</p><table><tr><td>1</td></tr></table>");
        assert!(xhtml("&bogus;").is_err());
    }

    #[test]
    fn links_for_epub() {
        assert_eq!(epub_link("ch01-cup.html#x"), "ch01-cup.xhtml#x");
        assert_eq!(epub_link("#a"), "#a");
        assert_eq!(epub_link("figures/a.gif"), "figures/a.gif");
    }

    #[test]
    fn dates_and_ids() {
        assert_eq!(iso8601(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso8601(1_794_268_800), "2026-11-10T00:00:00Z");
        let u = book_uuid("Upstream");
        assert_eq!(u.len(), 36);
        assert_eq!(u, book_uuid("Upstream"));
    }

    #[test]
    fn sentence_context() {
        let s = "First sentence here. The {{brew ratio}} is the water per gram of coffee, used throughout this book. Next.";
        let at = s.find("{{").unwrap();
        let c = render::sentence_around(s, at, at + 15);
        assert!(c.starts_with("The {{brew ratio}}") && c.ends_with("book."), "{c}");
    }
}

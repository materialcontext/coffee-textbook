//! upstream: builds a static, script-optional textbook that runs on
//! 2006-era hardware and browsers.
//!
//! The program is the build tool. The book is the folder it writes; open
//! `index.html` in that folder with any browser.
//!
//! Usage:
//!   upstream                                 find `content`, build into `dist`, report where
//!   upstream build [content-dir] [out-dir]   build, then run all checks
//!   upstream release [content-dir] [out-dir]  build, then the EPUB, PDF and web zip
//!   upstream lint  [content-dir]             prose lint only
//!   --buyer "Name"                           print the buyer's name in every page and edition
//!   upstream widgets                         list widget kinds
//!   upstream help                            this text

mod domain;
mod export;
mod lint;
mod parse;
mod render;
mod site;
mod widgets;

use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const HELP: &str = "upstream builds the Upstream textbook.

  upstream                                 find the content folder and build the book
  upstream build [content-dir] [out-dir]   same, with explicit folders
  upstream release [content-dir] [out-dir] build, then write the EPUB, the PDF (needs
                                           WeasyPrint) and a zip of the web book to a
                                           release folder beside the book folder
  --buyer \"Name\"                           add to build or release: licensed-to line
                                           in every page, the EPUB and the PDF
  upstream lint  [content-dir]             check prose only
  upstream widgets                         list calculator kinds

The book is written to a folder (default: dist next to content).
Open index.html in that folder with any web browser.";

/// Folders to try, in order, when no content folder is named:
/// the current directory, the executable's directory, and the project root
/// two or three levels above it (where cargo puts target/release/upstream).
fn find_content() -> Option<PathBuf> {
    let mut roots = vec![std::env::current_dir().ok()];
    if let Ok(exe) = std::env::current_exe() {
        let mut dir = exe.parent().map(Path::to_path_buf);
        for _ in 0..4 {
            roots.push(dir.clone());
            dir = dir.and_then(|d| d.parent().map(Path::to_path_buf));
        }
    }
    roots
        .into_iter()
        .flatten()
        .map(|r| r.join("content"))
        .find(|c| c.join("book.toml").is_file())
}

fn content_or_find(arg: Option<&&str>) -> Result<PathBuf, String> {
    match arg {
        Some(p) => Ok(PathBuf::from(p)),
        None => find_content().ok_or_else(|| {
            "no content folder found. Run upstream from the project folder (the one that contains \
             `content`), or name it: upstream build <content-dir> <out-dir>"
                .to_string()
        }),
    }
}

/// Keep a double-clicked console window open long enough to read.
fn pause() {
    print!("\nPress Enter to close.");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().lock().read_line(&mut String::new());
}

fn main() -> ExitCode {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let buyer = match args.iter().position(|x| x == "--buyer") {
        Some(i) if i + 1 < args.len() => {
            let name = args.remove(i + 1);
            args.remove(i);
            Some(name)
        }
        Some(_) => {
            eprintln!("--buyer needs a name, e.g. --buyer \"A. Reader\"");
            return ExitCode::from(2);
        }
        None => None,
    };
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    let interactive = a.is_empty() && buyer.is_none();
    let result = match a.as_slice() {
        [] | ["build" | "release"] | ["build" | "release", _] | ["build" | "release", _, _] => {
            let release = a.first() == Some(&"release");
            let rest = if a.is_empty() { &a[..] } else { &a[1..] };
            content_or_find(rest.first()).and_then(|content| {
                let out = rest
                    .get(1)
                    .map(PathBuf::from)
                    .unwrap_or_else(|| content.parent().unwrap_or(Path::new(".")).join("dist"));
                build(&content, &out, buyer.clone(), release)
            })
        }
        ["lint"] | ["lint", _] => content_or_find(a.get(1)).and_then(|c| lint_only(&c)),
        ["widgets"] => {
            widgets::kinds().iter().for_each(|k| println!("{k}"));
            Ok(true)
        }
        ["help"] | ["-h"] | ["--help"] | ["/?"] => {
            println!("{HELP}");
            Ok(true)
        }
        _ => {
            eprintln!("{HELP}");
            return ExitCode::from(2);
        }
    };
    let code = match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    };
    if interactive {
        pause();
    }
    code
}

fn lint_only(content: &Path) -> Result<bool, String> {
    let book = site::load(content)?;
    let built = site::build_pages(&book, "", "", &Default::default())?;
    let r = site::check(content, &book, &built.pages)?;
    for (file, f) in &r.prose {
        println!("{file}:{}: [{}] {}", f.line, f.rule, f.excerpt);
    }
    Ok(r.prose.is_empty())
}

fn build(content: &Path, out: &Path, buyer: Option<String>, release: bool) -> Result<bool, String> {
    let mut book = site::load(content)?;
    book.meta.buyer = buyer;
    let read = |f: &str| std::fs::read_to_string(content.join(f)).map_err(|e| format!("{f}: {e}"));
    let (license, readme) = (read("license.md")?, read("readme.txt")?);
    let changes = read("changes.md").unwrap_or_default();
    let dims = site::figure_sizes(content, &book);
    let mut built = site::build_pages(&book, &license, &changes, &dims)?;
    let report = site::check(content, &book, &built.pages)?;

    let mut ok = true;
    for (label, list) in [
        ("prose", &report.prose),
        ("citation", &report.citations),
        ("compat", &report.compat),
        ("glyph", &report.glyphs),
        ("figure", &report.figures),
        ("term", &report.terms),
    ] {
        for (file, f) in list.iter() {
            println!("{label} {file}:{}: [{}] {}", f.line, f.rule, f.excerpt);
            ok = false;
        }
    }
    for (name, n) in &report.sizes {
        let budget = if name == "print.html" { site::PRINT_BUDGET } else { site::PAGE_BUDGET };
        if *n > budget {
            println!("size {name}: {n} bytes exceeds budget {budget}");
            ok = false;
        }
    }
    // The flashcard file is a download, not a page, so it sits outside the page budget.
    let cards = export::flashcards(&book, &built.rendered, &built.definitions);
    let card_count = cards.lines().filter(|l| !l.starts_with('#')).count();
    built.pages.push(("flashcards.txt".to_string(), cards));
    let fig_bytes = site::write(content, out, &book, &built.pages, &[("README.txt", &readme), ("LICENSE.txt", &license)])?;
    let total = report.sizes.iter().map(|(_, n)| n).sum::<usize>() + fig_bytes;
    if total > site::SITE_BUDGET {
        println!("size: site {total} bytes exceeds budget {}", site::SITE_BUDGET);
        ok = false;
    }
    let out_abs = absolute(out);
    println!(
        "built {} pages and {} figures, {} bytes, {} chapters and {} appendices, {card_count} flashcards into {}",
        built.pages.len() + 2,
        site::figures(&book).len(),
        total,
        book.chapters.len(),
        book.appendices.len(),
        out_abs.display()
    );
    if release {
        ok &= release_editions(content, &out_abs, &book, &built)?;
    }
    println!("open this file in a web browser to read the book:\n  {}", out_abs.join("index.html").display());
    if !ok {
        println!("(the book was written, but the checks above need attention before release)");
    }
    Ok(ok)
}

/// Plain absolute path; canonicalize would give \\?\ paths on Windows.
fn absolute(p: &Path) -> PathBuf {
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir().map(|d| d.join(p)).unwrap_or_else(|_| p.to_path_buf())
    }
}

/// Seconds since 1970 for the EPUB date: SOURCE_DATE_EPOCH if set (reproducible builds), else now.
fn build_time() -> u64 {
    std::env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0))
}

/// EPUB, PDF and a zip of the web book, in a `release` folder beside the book folder.
fn release_editions(content: &Path, out: &Path, book: &domain::Book, built: &site::Built) -> Result<bool, String> {
    let m = &book.meta;
    let dir = out.parent().unwrap_or(Path::new(".")).join("release");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let stem = format!("{}-{}", m.title.to_lowercase().replace(' ', "-"), m.version.as_deref().unwrap_or(&m.edition.to_lowercase()));
    let figs: Vec<(String, Vec<u8>)> = site::figures(book)
        .into_iter()
        .filter_map(|f| std::fs::read(content.join("figures").join(&f)).ok().map(|b| (f, b)))
        .collect();

    let mut epub = export::epub(&export::EpubInput {
        book,
        rendered: &built.rendered,
        contents: &built.contents,
        glossary: &built.glossary,
        terms: &built.terms,
        license: &built.license,
        changes: &built.changes,
        css: site::CSS,
        modified: &export::iso8601(build_time()),
    })?;
    epub.extend(figs.iter().map(|(f, b)| (format!("OEBPS/figures/{f}"), b.clone())));
    let epub_path = dir.join(format!("{stem}.epub"));
    site::write_zip(&epub_path, &epub)?;
    println!("wrote {}", epub_path.display());

    let web_path = dir.join(format!("{stem}-web.zip"));
    site::write_zip(&web_path, &site::folder_files(out, &stem)?)?;
    println!("wrote {}", web_path.display());

    let pdf_src = out.join(".pdf-source.html");
    let doc = export::pdf_document(book, &built.rendered, &built.glossary, &built.index, site::CSS);
    std::fs::write(&pdf_src, doc).map_err(|e| e.to_string())?;
    let pdf_path = dir.join(format!("{stem}.pdf"));
    let ran = std::process::Command::new("weasyprint")
        .arg("--quiet")
        .arg(&pdf_src)
        .arg(&pdf_path)
        .status();
    let _ = std::fs::remove_file(&pdf_src);
    match ran {
        Ok(s) if s.success() => {
            println!("wrote {}", pdf_path.display());
            Ok(true)
        }
        Ok(s) => {
            println!("pdf: weasyprint failed ({s})");
            Ok(false)
        }
        Err(_) => {
            println!("pdf: skipped; install WeasyPrint (pip install weasyprint) to typeset the PDF");
            Ok(true)
        }
    }
}

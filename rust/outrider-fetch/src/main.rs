//! outrider-fetch: resilient URL fetch + text extraction.
//!
//! The Rust beginning of outrider's hot path. Pins the estate resilience
//! stack: fetch-kit (retries + backoff) wrapped around a breaker
//! CircuitBreaker. Python channels shell out to this binary; backend
//! registration follows in the web channel.
//!
//! Usage:
//!   outrider-fetch <url> [--json]
//!
//! Output: title + extracted text on stdout (JSON with --json).
//! Exit codes: 0 ok, 1 fetch/parse failure, 2 usage.

use serde::Serialize;
use std::time::Duration;


#[derive(Serialize)]
struct FetchResult {
    url: String,
    status: u16,
    title: String,
    chars: usize,
    text: String,
}

fn extract(html: &str) -> (String, String) {
    let doc = scraper::Html::parse_document(html);
    let title = doc
        .select(&scraper::Selector::parse("title").unwrap())
        .next()
        .map(|t| t.text().collect::<String>().trim().to_string())
        .unwrap_or_default();
    // v0.1: text from main-content tags only. This deliberately misses
    // div-wrapped raw text; a real readability pass (boilerplate scoring,
    // tag-scoped walk) is ROADMAP R1 work, not hidden in here.
    let sel = scraper::Selector::parse(
        "p,h1,h2,h3,h4,h5,h6,li,td,th,blockquote,pre,figcaption,dd,dt",
    )
    .unwrap();
    let mut out: Vec<String> = Vec::new();
    let mut last: Option<String> = None;
    for el in doc.select(&sel) {
        let t = el.text().collect::<String>();
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if t.is_empty() || last.as_deref() == Some(t.as_str()) {
            continue;
        }
        last = Some(t.clone());
        out.push(t);
    }
    (title, out.join("\n"))
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let as_json = args.iter().any(|a| a == "--json");
    let url = args.iter().find(|a| !a.starts_with("--")).cloned();
    let Some(url) = url else {
        eprintln!("usage: outrider-fetch <url> [--json]");
        std::process::exit(2);
    };

    // fetch-kit 0.2.2 wires retry + circuit breaker into its middleware
    // stack; the explicit breaker handle returns when a with_breaker-style
    // builder lands in a published release.
    let client = fetch_kit::Client::builder()
        .user_agent("outrider-fetch/0.1 (+https://github.com/WyattAu/outrider)")
        .timeout(Duration::from_secs(30))
        .retries(3)
        .build();

    let response = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("fetch failed: {e}");
            std::process::exit(1);
        }
    };
    let status = response.status().as_u16();
    if !(200..300).contains(&status) {
        eprintln!("http {status} for {url}");
        std::process::exit(1);
    }
    let body = match response.text().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("body read failed: {e}");
            std::process::exit(1);
        }
    };
    let (title, text) = extract(&body);

    if as_json {
        let out = FetchResult {
            url,
            status,
            title,
            chars: text.chars().count(),
            text,
        };
        println!("{}", serde_json::to_string(&out).unwrap_or_else(|_| "{}".into()));
    } else {
        if !title.is_empty() {
            println!("{title}\n");
        }
        println!("{text}");
    }
}

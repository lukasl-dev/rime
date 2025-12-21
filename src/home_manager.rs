use std::collections::HashSet;
use std::io::Error;

const HOME_MANAGER_OPTIONS_URL: &str =
    "https://nix-community.github.io/home-manager/options.xhtml";
const HOME_MANAGER_RESULTS_LIMIT: usize = 20;
const HOME_MANAGER_DESCRIPTION_LIMIT: usize = 200;

#[derive(Debug)]
pub(crate) struct HomeManagerOption {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) type_info: String,
}

fn decode_html_entities(input: &str) -> String {
    let mut out = input.to_string();
    for (from, to) in [
        ("&nbsp;", " "),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&amp;", "&"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&#x27;", "'"),
    ] {
        if out.contains(from) {
            out = out.replace(from, to);
        }
    }
    out
}

fn clean_html_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;

    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                out.push(' ');
            }
            _ => {
                if !in_tag {
                    out.push(ch);
                }
            }
        }
    }

    let decoded = decode_html_entities(&out);
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate_text(input: &str, max_chars: usize) -> String {
    let mut out = String::new();
    let mut chars = input.chars();

    for _ in 0..max_chars {
        if let Some(ch) = chars.next() {
            out.push(ch);
        } else {
            return out;
        }
    }

    if chars.next().is_some() {
        out.push_str("...");
    }

    out
}

fn extract_first_paragraph(dd_block: &str) -> Option<String> {
    let p_start = dd_block.find("<p")?;
    let tag_end = dd_block[p_start..].find('>')?;
    let content_start = p_start + tag_end + 1;
    let p_end = dd_block[content_start..].find("</p>")?;
    let content = &dd_block[content_start..content_start + p_end];
    let text = clean_html_text(content);
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn extract_description_before_type(text: &str) -> Option<String> {
    let type_idx = text.find("Type:")?;
    let desc = text[..type_idx].trim();
    if desc.is_empty() {
        None
    } else {
        Some(desc.to_string())
    }
}

fn extract_description(dd_block: &str) -> String {
    if let Some(text) = extract_first_paragraph(dd_block) {
        return text;
    }

    let text = clean_html_text(dd_block);
    if let Some(desc) = extract_description_before_type(&text) {
        return desc;
    }

    text
}

fn extract_type_info(dd_block: &str) -> String {
    let text = clean_html_text(dd_block);
    let Some(type_idx) = text.find("Type:") else {
        return String::new();
    };

    let rest = text[type_idx + "Type:".len()..].trim_start();
    if rest.is_empty() {
        return String::new();
    }

    let mut end = rest.len();
    for marker in [
        "Default:",
        "Example:",
        "Declared by:",
        "Defined by:",
        "Related packages:",
        "Related options:",
    ] {
        if let Some(idx) = rest.find(marker) {
            if idx < end {
                end = idx;
            }
        }
    }

    rest[..end].trim().to_string()
}

fn extract_dd_block<'a>(html: &'a str, cursor: usize) -> Option<(&'a str, usize)> {
    let dd_rel = html[cursor..].find("<dd")?;
    let dd_start = cursor + dd_rel;
    let dd_tag_end_rel = html[dd_start..].find('>')?;
    let dd_content_start = dd_start + dd_tag_end_rel + 1;
    let dd_end_rel = html[dd_content_start..].find("</dd>")?;
    let dd_block = &html[dd_content_start..dd_content_start + dd_end_rel];
    let next_cursor = dd_content_start + dd_end_rel + "</dd>".len();
    Some((dd_block, next_cursor))
}

fn parse_home_manager_options(
    html: &str,
    query: &str,
    limit: usize,
) -> Vec<HomeManagerOption> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    let mut seen = HashSet::new();
    let mut cursor = 0;

    while let Some(id_offset) = html[cursor..].find("id=\"opt-") {
        let id_pos = cursor + id_offset;
        let id_start = id_pos + "id=\"".len();
        let Some(id_end_rel) = html[id_start..].find('"') else {
            break;
        };
        let id_val = &html[id_start..id_start + id_end_rel];
        cursor = id_start + id_end_rel;

        let Some(name_raw) = id_val.strip_prefix("opt-") else {
            continue;
        };
        let name = name_raw.replace("_name_", "<name>");
        if !name.to_lowercase().contains(&query_lower) {
            continue;
        }
        if !seen.insert(name.clone()) {
            continue;
        }

        let mut description = String::new();
        let mut type_info = String::new();
        let mut next_cursor = cursor;

        if let Some((dd_block, dd_next_cursor)) = extract_dd_block(html, cursor) {
            description = extract_description(dd_block);
            type_info = extract_type_info(dd_block);
            next_cursor = dd_next_cursor;
        }

        let description = if description.is_empty() {
            description
        } else {
            truncate_text(description.trim(), HOME_MANAGER_DESCRIPTION_LIMIT)
        };

        results.push(HomeManagerOption {
            name,
            description,
            type_info: type_info.trim().to_string(),
        });

        cursor = next_cursor;
        if results.len() >= limit {
            break;
        }
    }

    results
}

pub(crate) fn search_home_manager_options(
    query: &str,
) -> Result<Vec<HomeManagerOption>, Error> {
    let query = query.trim();
    if query.is_empty() {
        return Err(Error::other("query must not be empty"));
    }

    let resp = ureq::get(HOME_MANAGER_OPTIONS_URL)
        .set(
            "User-Agent",
            "rime/1.0 (+https://github.com/lukasl-dev/rime)",
        )
        .set("Accept", "text/html")
        .call()
        .map_err(|err| Error::other(format!("home manager request failed: {err}")))?;

    let body = resp
        .into_string()
        .map_err(|err| Error::other(format!("home manager response read failed: {err}")))?;

    Ok(parse_home_manager_options(
        &body,
        query,
        HOME_MANAGER_RESULTS_LIMIT,
    ))
}

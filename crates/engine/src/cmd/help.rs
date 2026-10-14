//! Help → project links (the website, this app's page and its
//! GitHub repository). The commands return the URL; the frontend opens it.

use serde_json::{Value, json};

use super::*;

/// The app's id on getartcraft.com and GitHub.
pub const APP_ID: &str = "vectorcraft";
pub const WEBSITE_URL: &str = "https://freeandiy.freeandy.agency";

/// This app's page on the website.
pub fn app_page_url() -> String {
    format!("{WEBSITE_URL}/vector")
}

/// This app's source repository.
pub fn github_url() -> String {
    "https://github.com/FreeAndy21/freeandiy-vector".to_string()
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.website", "FreeANDIY Website", ["Help"], None, "{} → {url}", always, |_, _| url(WEBSITE_URL.into())),
        cmd!(query "help.appPage", "FreeANDIY Vector on freeandiy.freeandy.agency", ["Help"], None, "{} → {url} this app's page", always, |_, _| url(app_page_url())),
        cmd!(query "help.github", "FreeANDIY Vector on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {website, appPage, github}", always, |_, _| {
            Ok(json!({ "website": WEBSITE_URL, "appPage": app_page_url(), "github": github_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.appPage", &json!({})).unwrap()["url"], "https://freeandiy.freeandy.agency/vector");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/FreeAndy21/freeandiy-vector");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["website"], "https://freeandiy.freeandy.agency");
    }
}

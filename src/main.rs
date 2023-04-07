use scraper::{Html, Selector};
use std::collections::HashSet;

// note: thirtyfour is selenium driver, so that is an alternative
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut next_wikis: Vec<String> = Vec::new();

    // note, if need ot make general then look into way to not have constants into the strings
    let base_url = String::from("https://wiki.borderconnect.com");

    next_wikis.push("/index.php".to_owned());

    let mut visited: HashSet<String> = HashSet::new();

    let help_guide_links_selector = Selector::parse("#p-Help_Guides a").unwrap();

    // collect next ones to visit in this run
    // finish this run first
    // decrement "depth"
    // exit on depth = 0 or all links visited

    while !next_wikis.is_empty() {
        let cur_wiki = next_wikis.pop().unwrap();
        if visited.contains(&cur_wiki) {
            continue;
        }

        println!("{}", base_url.clone() + &cur_wiki);
        let resp = reqwest::get(base_url.clone() + &cur_wiki)
            .await?
            .text()
            .await?;

        let dom = Html::parse_fragment(&resp);

        visited.insert(cur_wiki);

        // let dom = Document::from(resp.as_str());
        // gets first matched node of #mw-content-text and unwraps it
        println!(
            "{}",
            /*
            dom.find(predicate::Attr("id", "mw-content-text"))
               .next()
               .unwrap()
               .text()
               .trim() */
            dom.select(
                &Selector::parse("#mw-content-text :not(h1):not(.references):not(.toc)").unwrap()
            )
            .map(|node| node.text().next().unwrap_or("").to_string())
            .reduce(|accum, item| accum + &item + "\n")
            .unwrap(),
        );

        // only get relative links of href attributes of a links under
        // the "Help Guides" page

        /* let someiter = dom
            .find(predicate::Attr("id", "p-Help_Guides"))
            .next() // going to the only result here
            .unwrap()
            .clone()
            .find(predicate::Name("a")); // find all "a" tags below this node

        let links: Vec<String> = someiter
            .map(|node| node.clone().attr("href").unwrap().to_string())
            .filter(|node| node.starts_with('/'))
            .collect();
        */

        let links = dom
            .select(&help_guide_links_selector)
            .map(|node| node.value().attr("href").unwrap().to_string())
            .filter(|node| node.starts_with('/'));

        next_wikis.extend(links);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use scraper::{Html, Selector};

    #[test]
    fn test_select() -> Result<(), Box<dyn std::error::Error>> {
        let input = reqwest::blocking::get("https://www.google.com")
            .unwrap()
            .text()
            .unwrap()
            .to_string();

        let dom = Html::parse_fragment(&input);

        println!(
            "{}",
            dom.select(&Selector::parse("html :not(h1):not(.references):not(.toc)").unwrap())
                .map(|node| node.text().next().unwrap_or("").to_string())
                .reduce(|accum, item| accum + &item + "\n")
                .unwrap()
        );

        Ok(())
    }
}

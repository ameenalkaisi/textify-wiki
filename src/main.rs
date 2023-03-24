use select::{document::Document, predicate};
use std::collections::{HashSet, VecDeque};

// note: thirtyfour is selenium driver, so that is an alternative
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut next_wikis: VecDeque<String> = VecDeque::new();
    next_wikis.push_back(
        "https://wiki.borderconnect.com/index.php/Welcome_to_BorderConnect%27s_Support_Wiki"
            .to_string(),
    );

    let mut visited: HashSet<String> = HashSet::new();

    // collect next ones to visit in this run
    // finish this run first
    // decrement "depth"
    // exit on depth = 0 or all links visited

    while !next_wikis.is_empty() {
        let cur_wiki = next_wikis.pop_front().unwrap();
        if visited.contains(&cur_wiki) {
            continue;
        }

        let resp = reqwest::get(&cur_wiki).await?.text().await?;

        visited.insert(cur_wiki);

        let dom = Document::from(resp.as_str());
        for node in dom.find(predicate::Text) {
            if node.html().chars().any(|c| !c.is_whitespace()) {
                let cur_str: String = node.html().chars().filter(|c| !c.is_whitespace()).collect();

                println!("{}", cur_str);
            }
        }

        todo!("figure out a way to display all the info properly here");
    }

    Ok(())
}

use std::{env, io};
use serde::Deserialize;

extern crate alfred;
extern crate reqwest;
extern crate serde;

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
struct Repo {
	name: String,
	description: Option<String>,
	html_url: String,
	stargazers_count: i32
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
struct Response {
    items: Vec<Repo>,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let pattern = args.get(0).unwrap();
    let url = format!(
        "https://api.github.com/search/repositories?q={}&sort=score&order=desc",
        pattern
    );
    let resp: Response = reqwest::get(&url).unwrap().json().unwrap();
    let repos = resp.items;
    let alfred_items: Vec<_> = repos
        .iter()
        .map(|repo| {
            let stars = &repo.stargazers_count;
            let emoji = if stars >= &10000 { "🚀" }
                else if stars >= &5000 { "😁" }
                else if stars >= &1000 { "😃" }
                else if stars >= &200 { "😌" }
                else { "😭" };
            alfred::ItemBuilder::new(format!("{} {} ({})", emoji, &repo.name, stars))
                .subtitle(repo.description.clone().unwrap_or("".to_string()))
                .arg(&repo.html_url)
                .into_item()
        })
        .collect();

    alfred::json::write_items(io::stdout(), &alfred_items).unwrap();
}

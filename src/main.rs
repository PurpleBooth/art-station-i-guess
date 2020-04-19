use futures::future::join_all;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;

use clap::{App, Arg};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct ProjectsItem {
    hash_id: String,
}

type ProjectsData = Vec<ProjectsItem>;

#[derive(Serialize, Deserialize, Debug)]
struct ProjectsResponse {
    data: ProjectsData,
    total_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectAsset {
    image_url: String,
}

type ProjectAssets = Vec<ProjectAsset>;

#[derive(Serialize, Deserialize, Debug)]
struct ProjectResponse {
    assets: ProjectAssets,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(env!("APP_NAME"))
        .version(env!("VERSION"))
        .author(env!("AUTHOR_EMAIL"))
        .about("Download a bunch of art for wallpaper from art station.")
        .arg(
            Arg::with_name("username")
                .help("Provide this to rerun a previous configuration")
                .index(1)
                .required(true),
        )
        .get_matches();

    let username: String = matches.value_of("username").unwrap().to_string();
    let client = Client::new();
    let projects_url = format!(
        "https://www.artstation.com/users/{}/projects.json",
        username
    );
    let project_list = client
        .get(&projects_url)
        .send()
        .await?
        .json::<ProjectsResponse>()
        .await?;

    let project_urls = project_list
        .data
        .into_iter()
        .map(|project| {
            format!(
                "https://www.artstation.com/projects/{}.json",
                project.hash_id
            )
        })
        .collect::<Vec<_>>();

    let client = Client::new();
    let project_requests = project_urls
        .into_iter()
        .map(|url| client.get(&url).send())
        .collect::<Vec<_>>();

    let project_responses = join_all(project_requests)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .map(|response| response.json::<ProjectResponse>());

    let projects = join_all(project_responses).await;

    let asset_url = projects
        .into_iter()
        .filter_map(Result::ok)
        .map(|x| x.assets)
        .flatten()
        .map(|x| x.image_url)
        .collect::<Vec<_>>();

    let image_requests = asset_url
        .iter()
        .cloned()
        .map(|url| client.get(&url).send())
        .collect::<Vec<_>>();

    let image_responses = join_all(image_requests).await;

    for (index, image_response) in image_responses.into_iter().enumerate() {
        let image = image_response.unwrap();
        let image_bytes = image.bytes().await?;

        let mut path = std::env::current_dir()?;
        path.push(format!("{}.jpg", index));
        let display = path.display();

        let mut file = match File::create(&path.as_path()) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(&image_bytes) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    Ok(())
}

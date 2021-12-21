use std::collections::HashMap;

use anyhow::*;
use clap::{App, AppSettings, Arg, SubCommand};
use colored::Colorize;
use reqwest::{Response, Url};

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl TryFrom<&str> for KvPair {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split("=");
        Ok(Self {
            k: split.next().ok_or(anyhow!("no key"))?.to_string(),
            v: split.next().ok_or(anyhow!("no value"))?.to_string(),
        })
    }
}

async fn get(url: &str) -> Result<Response> {
    Ok(reqwest::get(url).await?)
}

async fn post(url: &str, params: &[KvPair]) -> Result<Response> {
    let client = reqwest::Client::new();

    let mut kvs = HashMap::new();
    params.iter().for_each(|kv| {
        kvs.insert(&kv.k, &kv.v);
    });

    Ok(client.post(url).form(&kvs).send().await?)
}

fn print_header(resp: &Response) {
    print!(
        "{}",
        format!("{:?} {:?} ", resp.version(), resp.status()).blue()
    );
    print!("{}", resp.status().canonical_reason().unwrap().cyan());
    print!("\n");

    let headers = resp.headers();
    for (k, v) in headers {
        println!("{}: {}", k.as_str().cyan(), v.to_str().unwrap());
    }
}

async fn print_body(resp: Response) -> Result<()> {
    let content_type = resp
        .headers()
        .get("content-type")
        .map(|v| v.to_str().unwrap().to_string());

    let text = resp.text().await?;
    match content_type {
        Some(ty) if ty == "application/json" => {
            println!(
                "{}",
                jsonxf::pretty_print(&text).map_err(|err| anyhow!(err))?
            );
        }
        _ => {
            println!("{}", text);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("HTTPie")
        .version("1.0")
        .author("someone")
        .about("a http requester")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("get")
                .about("send a get request")
                .arg(
                    Arg::with_name("url")
                        // .index(1)
                        .help("the url")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("post")
                .about("send a post request")
                .arg(Arg::with_name("url").help("the url").required(true))
                .arg(
                    Arg::with_name("kvpairs")
                        .help("the kv pairs")
                        .multiple(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(subcmd_get)) => {
            let url = subcmd_get.value_of("url").unwrap();
            let _: Url = url.parse()?;
            let resp = get(url).await?;
            print_header(&resp);
            print!("\n");
            print_body(resp).await?;
        }

        ("post", Some(subcmd_post)) => {
            let url = subcmd_post.value_of("url").unwrap();
            let _: Url = url.parse()?;

            let mut kvpairs: Vec<KvPair> = Vec::new();
            if let Some(values) = subcmd_post.values_of("kvpairs") {
                for v in values {
                    kvpairs.push(KvPair::try_from(v)?);
                }
            }
            let resp = post(url, &kvpairs).await?;
            print_header(&resp);
            print!("\n");
            print_body(resp).await?;
        }

        _ => unreachable!(),
    }
    Ok(())
}

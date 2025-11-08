use std::sync::Arc;
use std::time::Instant;

use aes::Aes256;
use aes::cipher::{BlockDecryptMut, KeyIvInit, block_padding::Pkcs7};
use cbc::Decryptor;
use clap::Parser;
use clap_derive::Subcommand;
use reqwest::cookie::Jar;
use reqwest::{Client, Url};
use serde::Deserialize;

use crate::Day;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=25))]
    pub day: Option<u16>,
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=3))]
    pub part: Option<u16>,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Download {
        #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=25))]
        day: u16,
    },
    Cookie {
        cookie: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
struct UserInfo {
    seed: u16,
}

#[derive(Debug, Clone, Deserialize)]
struct InputData {
    #[serde(rename = "1", with = "hex::serde")]
    first: Vec<u8>, // TODO: Make optional
    #[serde(rename = "2", with = "hex::serde")]
    second: Vec<u8>,
    #[serde(rename = "3", with = "hex::serde")]
    third: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
struct Keys {
    key1: Option<String>,
    key2: Option<String>,
    key3: Option<String>,
    // answer1: Option<String>,
    // answer2: Option<String>,
    // answer3: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Runner {
    cookie: Option<Arc<str>>,
    seed: Option<u16>,
}

impl Runner {
    pub fn save_cookie(&mut self, new_cookie: &str) {
        let cookie_fn = "./input/cookie.txt";
        std::fs::write(cookie_fn, new_cookie).expect("Write cookie file");
        self.cookie = Some(Arc::from(new_cookie));
    }
    fn get_cookie(&mut self) -> Arc<str> {
        if let Some(cookie) = &self.cookie {
            return cookie.clone();
        }
        let cookie_fn = "./input/cookie.txt";
        if std::fs::exists(cookie_fn).unwrap() {
            self.cookie = Some(Arc::from(std::fs::read_to_string(cookie_fn).unwrap()));
            return self.cookie.as_ref().unwrap().clone();
        }
        panic!("Cookie not found. Please use the `cookie` subcommand to set it");
    }
    fn cli_with_cookie(&mut self) -> Client {
        let jar = Jar::default();
        jar.add_cookie_str(
            format!("everybody-codes={}", self.get_cookie()).as_str(),
            &Url::parse("https://everybody.codes/").expect("Valid url"),
        );
        Client::builder()
            .cookie_provider(Arc::new(jar))
            .build()
            .expect("Build Client")
    }
    async fn get_seed(&mut self) {
        let cli = self.cli_with_cookie();

        let resp = cli
            .get("https://everybody.codes/api/user/me")
            .send()
            .await
            .expect("request failed");

        let user_info = resp.json::<UserInfo>().await.expect("json");

        self.seed = Some(user_info.seed);
    }
    pub async fn download(&mut self, day: u16) {
        let cli = self.cli_with_cookie();

        let keyresp = cli
            .get(format!(
                "https://everybody.codes/api/event/2025/quest/{day}"
            ))
            .send()
            .await
            .expect("request failed");

        let keys = keyresp.json::<Keys>().await.expect("Request failed");

        if self.seed.is_none() {
            self.get_seed().await;
        }
        let seed = self.seed.as_ref().expect("seed");

        let resp = cli
            .get(format!(
                "https://everybody-codes.b-cdn.net/assets/2025/{day}/input/{seed}.json"
            ))
            .send()
            .await
            .expect("request failed");

        let input = resp.json::<InputData>().await.expect("json");
        for ((contents, key), part) in [
            (&input.first, &keys.key1),
            (&input.second, &keys.key2),
            (&input.third, &keys.key3),
        ]
        .into_iter()
        .zip(1..)
        {
            let Some(key) = key else {
                println!("No key for part {part}. Skipping.");
                continue;
            };
            let key_bytes = key.as_bytes();
            let iv = &key_bytes[..16];
            let cihper = Decryptor::<Aes256>::new(key_bytes.into(), iv.into());
            let mut buf = vec![0_u8; contents.len()];
            let decrypted = cihper
                .clone()
                .decrypt_padded_b2b_mut::<Pkcs7>(contents, &mut buf)
                .expect("Decrypt input files");

            let filename = format!("./input/day_{day:02}_part_{part}.txt");
            std::fs::write(&filename, decrypted).expect("Write input files");
            println!("Saved {filename}");
        }
    }

    pub async fn run<D: Day>(&mut self, day: u16, part_filter: Option<u16>) {
        println!();
        for part in 1..=3 {
            if part_filter.is_none_or(|p| p == part) {
                let filename = format!("./input/day_{day:02}_part_{part}.txt");
                if !std::fs::exists(&filename).unwrap() {
                    self.download(day).await;
                }
                let time_start = Instant::now();
                let input_text = std::fs::read_to_string(filename).unwrap();
                let input = match D::parse(&input_text) {
                    Ok(input) => input,
                    Err(err) => {
                        println!("Parse error: {err}");
                        continue;
                    }
                };
                let time_parsed = Instant::now();
                match part {
                    1 => println!("Quest {day} - Part 1: {}", D::part_1(&input)),
                    2 => println!("Quest {day} - Part 2: {}", D::part_2(&input)),
                    _ => println!("Quest {day} - Part 3: {}", D::part_3(&input)),
                }
                let time_complete = Instant::now();
                println!(
                    "          parsing: {:?}",
                    time_parsed.duration_since(time_start)
                );
                println!(
                    "          runner: {:?}",
                    time_complete.duration_since(time_parsed)
                );
                println!();
            }
        }
    }
}

#![allow(unused)]
use reqwest;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use url::{Host, Url};

use std::sync::Arc;
use tokio::sync::Mutex;

mod crawler;
// use crawler::*;

mod domain;
use domain::*;

mod errors;
use errors::*;
type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
pub async fn run() -> Result<(), RError> {
    let args: Vec<String> = env::args().collect();
    let mut d = Domain::new(&args)?;
    d.process_domain_links().await?;
    let findings = d.indexables;
    let mut c = crawler::crawl(findings).await?;
    Ok(())
}

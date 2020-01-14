use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use chrono::{DateTime, SecondsFormat, Utc};
use failure::{err_msg, Fallible};
use git2::Repository;
use mac_address::get_mac_address;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct BreadCrumb {
    cwd: String,
    id: String,
    create_time: String,
    git_hash: String,
    git_repo: String,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Save context about data folder creation")]
struct Cli {
    #[structopt(long = "base")]
    pub base_folder: String,
}

fn create_output(base_folder: &str) -> Fallible<String> {
    // Gather the context for breadcrumbs
    let path = env::current_dir()?;
    let now: DateTime<Utc> = Utc::now();
    let git_hash = if let Ok(repo) = Repository::discover(".") {
        let head = repo.head()?.target().map(|s| s.to_string()).unwrap_or_default();
        head
    } else {
        "".to_string()
    };

    // create the output folder in the base folder, use an uuid
    let context = Context::new(1);
    let timestamp = Timestamp::from_unix(
        context,
        now.timestamp() as u64,
        now.timestamp_subsec_nanos(),
    );
    let node_id = get_mac_address()?
        .ok_or(err_msg("missing MAC address"))?
        .bytes();
    let uuid = Uuid::new_v1(timestamp, &node_id)?.to_string();
    let out_folder = Path::new(base_folder).join(uuid.clone());
    create_dir_all(out_folder.as_path())?;

    let bc = BreadCrumb {
        cwd: path.as_os_str().to_str().unwrap().to_string(),
        create_time: now.to_rfc3339_opts(SecondsFormat::Secs, false),
        id: uuid,
        git_hash,
        git_repo: "".to_string()
    };
    let json = serde_json::to_string_pretty(&bc)?;
    let out_file = out_folder.join("datacrumbs.json");
    let mut file = File::create(out_file)?;
    file.write(json.as_bytes())?;
    out_folder
        .to_str()
        .map(String::from)
        .ok_or(err_msg("bad path"))
}

fn main() -> Result<(), failure::Error> {
    let cli = Cli::from_args();
    let folder = create_output(&cli.base_folder)?;
    println!("{}", folder);
    Ok(())
}

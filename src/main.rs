use std::env;
use std::fs::{create_dir_all, File};

use chrono::{DateTime, SecondsFormat, Utc};
use failure::{err_msg, Fallible};
use git2::Repository;
use mac_address::get_mac_address;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct BreadCrumb {
    cwd: String,
    run_time: String,
    git_hash: String,
}

fn process(base_folder: &str) -> Fallible<String> {
    // Gather the context for breadcrumbs
    let path = env::current_dir()?;
    let now: DateTime<Utc> = Utc::now();
    let git_hash = Repository::open(".")
        .and_then(|repo| {
            repo.head()
                .map(|git_ref| git_ref.target().map(|t| t.to_string()).unwrap_or_default())
        })
        .unwrap_or_default();

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
    let uuid = Uuid::new_v1(timestamp, &node_id)?;
    let out_folder = Path::new(base_folder).join(uuid.to_string());
    create_dir_all(out_folder.as_path())?;

    let bc = BreadCrumb {
        cwd: path.as_os_str().to_str().unwrap().to_string(),
        run_time: now.to_rfc3339_opts(SecondsFormat::Secs, false),
        git_hash,
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
    let mut args = env::args();
    if args.len() != 2 {
        return Err(failure::err_msg("Pass in the base folder"));
    }
    let base_folder = args.nth(1).ok_or(err_msg("bad base folder"))?;
    let folder = process(&base_folder)?;
    println!("{}", folder);
    Ok(())
}

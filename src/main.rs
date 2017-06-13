extern crate envy;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate hyper_native_tls;
extern crate itertools;
extern crate url;

use hyper::Client;
use hyper::header::{Authorization, Basic};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use itertools::Itertools;
use url::Url;

pub mod errors {
    error_chain!{
        errors {
            InvalidRequest
        }
        foreign_links {
            Http(::hyper::Error);
            TLS(::hyper_native_tls::native_tls::Error);
            URL(::url::ParseError);
        }
    }
}
use errors::{Result, ResultExt};

#[derive(Deserialize)]
struct Config {
    jira_host: String,
    jira_username: String,
    jira_password: String,
    project: String,
}

#[derive(Deserialize, Debug)]
struct Version {
    id: String,
    description: Option<String>,
    name: String,
    archived: bool,
    released: bool,
    #[serde(rename="releaseDate")]
    release_date: Option<String>,
    overdue: Option<bool>,
    #[serde(rename="userReleaseDate")]
    user_release_date: Option<String>,
}

fn run() -> Result<()> {
    let config = envy::from_env::<Config>()
        .chain_err(|| "Invalid config")?;

    let url = Url::parse(
        &format!(
            "{host}//rest/api/2/project/{project}/versions",
            host = config.jira_host,
            project = config.project
        ),
    )?;
    let res = Client::with_connector(HttpsConnector::new(NativeTlsClient::new()?))
        .get(url)
        .header(
            Authorization(
                Basic {
                    username: config.jira_username,
                    password: Some(config.jira_password),
                },
            ),
        )
        .send()?;

    if !res.status.is_success() {
        return Err(errors::ErrorKind::InvalidRequest.into());
    }
    let versions = serde_json::from_reader::<_, Vec<Version>>(res)
        .chain_err(|| "failed to parse builds")?;

    // only released
    let released = versions
        .iter()
        .filter_map(|version| version.release_date.clone());
    let by_date = released.group_by(|date| date.clone());
    for (date, vs) in by_date.into_iter() {
        println!("{:?} {}", date, vs.count())
    }
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        use error_chain::ChainedError; // trait which holds `display`
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display()).expect(errmsg);
        ::std::process::exit(1);
    }
}

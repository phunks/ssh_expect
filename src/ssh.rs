

use regex::Regex;
use crate::inventory::{DiskStatus, ResultStatus, Server};
use crate::crypt::{decrypt};
use crate::config::read_config;
use async_ssh2_tokio::client::{Client, AuthMethod, ServerCheckMethod};
use anyhow::Result;
use futures::future::join_all;
use magic_crypt::generic_array::typenum::private::Trim;
use serde::{Deserialize, Serialize};
use tabled::{Tabled, Table};
use tabled::settings::{Alignment, Modify, Style};
use tabled::settings::location::ByColumnName;

pub async fn do_ssh() -> Result<String>  {
    let mut inventory = read_config("test.toml").unwrap();

    let result_status = join_all(&mut inventory
        .server
        .iter_mut()
        .map(|i| async {
            ssh_expect(i)
                .await
                .unwrap_or_else(|e| ResultStatus {
                host: i.host.to_string(),
                disk_status: vec![DiskStatus {
                    file_system: "n/a".to_string(),
                    total_space: "n/a".to_string(),
                    used_space : "n/a".to_string(),
                    free_space : "n/a".to_string(),
                    used_percentage: "n/a".to_string(),
                    mount_point: "n/a".to_string(),
                }],
            })
        })
    ).await;

    let mut v = Vec::new();
    for i in result_status {
        v.append(&mut i.from());
    }

    let table = Table::new(v)
        .with(Style::markdown())
        .with(Modify::new(ByColumnName::new("used_percentage"))
                .with(Alignment::right()))
        .to_string();

    Ok(table)
}

async fn ssh_expect(server: &mut Server) -> Result<ResultStatus> {
    let auth_method = AuthMethod::with_password(&decrypt(&server.user_pw));

    let mut client = Client::connect(
        (server.host.to_string(), server.port),
        &*server.user_id,
        auth_method,
        ServerCheckMethod::NoCheck,
    ).await?;

    // linux: "Filesystem,1M-blocks,Used,Available,Use%,Mounted on"
    // aix:  df -Pc FileSystem, TotalSpace, UsedSpace, FreeSpace, UsedPercentage, MountPoint
    let result = client.execute(
        &*format!("df -m {}", &server.observe_target.join(" "))).await?;
    assert_eq!(result.exit_status, 0);

    let mut v = Vec::new();
    v = result.stdout.split("\n").collect();

    let re = Regex::new(r"([ \t]+)").expect("Invalid regex");
    let mut r = ResultStatus::default();

    for i in 1..v.len() - 1 {
        let el: Vec<&str> = re.split(v[i]).collect();

        r.host = server.host.to_string();
        r.disk_status.append(&mut vec! {DiskStatus {
            file_system     : el[0].parse()?,
            total_space     : el[1].parse()?,
            used_space      : el[2].parse()?,
            free_space      : el[3].parse()?,
            used_percentage : el[4].replace("%", "").parse()?,
            mount_point     : el[5].parse()?,
        }})
    }
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    use crate::ssh::do_ssh;

    pub(crate) fn runtime() -> &'static Runtime {
        static RUNTIME: once_cell::sync::OnceCell<Runtime> = once_cell::sync::OnceCell::new();
        RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
        })
    }

    #[test]
    fn test_do_ssh() {
        runtime().block_on(async {
            let table = do_ssh().await.unwrap_or_else(|e| panic!("ssh failed with {}", e));
            println!("{table}");
        });
    }
}

use std::sync::Arc;
use crate::crypt::decrypt;
use crate::inventory::{DiskStatus, Inventory, ResultStatus, Server};
use anyhow::Result;
use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
use futures::future::join_all;
use regex::Regex;
use tabled::settings::location::ByColumnName;
use tabled::settings::{Alignment, Modify, Span, Style};
use tabled::Table;
use std::sync::Mutex;
use itertools::Itertools;
use tabled::settings::object::Cell;
use crate::Rc;

pub async fn do_ssh(inventory: &mut Inventory) -> Result<(String, Rc)> {
    let ssh_err = CollectError::default();
    let result_status = join_all(&mut inventory.server.iter_mut()
        .map(|i| async {
        ssh_expect(i, ssh_err.clone()).await.unwrap_or_else(|_e| ResultStatus {
            host: i.host.to_string(),
            disk_status: vec![DiskStatus::default()],
        })
    }))
    .await;

    let mut rc = Rc::Normal;
    let v = result_status
        .into_iter()
        .flat_map(|i| i.from())
        .collect::<Vec<_>>();

    let v_span = |r, c, span| Modify::new(Cell::new(r, c))
        .with(Span::row(span));

    let mut table = Table::new(&v)
        .with(Style::markdown())
        .with(Modify::new(ByColumnName::new("used_percentage"))
            .with(Alignment::right())).to_owned();

    let mut n = 1;
    v.into_iter().map(|x|x.host).dedup_with_count().for_each(|i|{
        table.with(v_span(n, 0, i.0 as isize));
        n += i.0;
    });

    let mut table = table.to_string();
    if table.contains("⚠️") { rc = Rc::Warn; }
    if ssh_err.len().ne(&0) {
        rc = Rc::Error;
        table = format!("{}{}", table, ssh_err.write());
    };

    Ok((table, rc))
}

#[derive(Default, Clone)]
struct CollectError(Arc<Mutex<Vec<String>>>);

impl CollectError {
    fn push(&mut self, err: &str) {
        self.0.lock().unwrap().push(err.into());
    }
    fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }
    fn write(&self) -> String {
        format!("\n\n⚠️ {}", self.0.lock().unwrap().join("\n⚠️ "))
    }
}

async fn ssh_expect(server: &mut Server, mut ssh_err: CollectError) -> Result<ResultStatus, > {
    let auth_method = AuthMethod::with_password(&decrypt(&server.user_pw));

    let client = Client::connect(
        (server.host.to_string(), server.port),
        &server.user_id,
        auth_method,
        ServerCheckMethod::NoCheck,
    ).await;

    let client = match client {
        Ok(c) => c,
        Err(e) => {
            ssh_err.push(&format!("{}: {}", server.host, e));
            return Err(anyhow::Error::from(e));
        }
    };

    // linux: df -m "Filesystem, 1M-blocks, Used, Available, Use%, Mounted on"
    // aix:   df -Pcm "FileSystem, TotalSpace, UsedSpace, FreeSpace, UsedPercentage, MountPoint"
    let aaa = &*format!(
        "df -m {}",
        &server
            .target
            .iter()
            .map(|x| x.mount_point.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    );
    let result = client.execute(aaa).await?;

    let stdout = result.stdout.split("\n").collect::<Vec<&str>>();
    let stderr = result.stderr.split("\n").collect::<Vec<&str>>();
    stderr.into_iter().filter(|&x|!x.is_empty())
        .for_each( |x| { ssh_err.push(&format!("{}: {}", server.host, x))});

    let re = Regex::new(r"([ \t:]+)").expect("Invalid regex");
    let mut r = ResultStatus::default();

    #[allow(clippy::needless_range_loop)]
    for i in 1..stdout.len() - 1 {
        let el: Vec<&str> = re.split(stdout[i]).collect();

        r.host = server.host.to_string();
        r.disk_status.append(&mut vec![DiskStatus {
            file_system: el[0].into(),
            total_space: el[1].into(),
            used_space: el[2].into(),
            free_space: el[3].into(),
            used_percentage: el[4].replace("%", ""),
            mount_point: el[5].into(),
            target: server
                .clone()
                .target
                .into_iter()
                .find(|x| x.mount_point == el[5])
                .unwrap(),
        }])
    }
    Ok(r)
}

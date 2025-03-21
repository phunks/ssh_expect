use serde::{Deserialize, Serialize};
use tabled::Tabled;
use better_default::Default;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Inventory {
    pub server: Vec<Server>,
    pub webhook: WebHook,
}
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub user_id: String,
    pub user_pw: String,
    pub target: Vec<Target>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Target {
    pub mount_point: String,
    pub threshold: i8,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct DiskStatus {
    #[default("n/a".into())]
    pub file_system: String,
    #[default("n/a".into())]
    pub total_space: String,
    #[default("n/a".into())]
    pub used_space: String,
    #[default("n/a".into())]
    pub free_space: String,
    #[default("n/a".into())]
    pub used_percentage: String,
    #[default("n/a".into())]
    pub mount_point: String,
    pub target: Target,
}


#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ResultStatus {
    pub host: String,
    pub disk_status: Vec<DiskStatus>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct WebHook {
    pub url: String,
}

#[derive(Default, Debug, Clone, Tabled, Deserialize, Serialize)]
pub struct MarkdownTable {
    pub host: String,
    #[tabled(rename = "file system")]
    pub file_system: String,
    #[tabled(rename = "total space (MB)")]
    pub total_space: String,
    #[tabled(rename = "used space (MB)")]
    pub used_space: String,
    #[tabled(rename = "free space (MB)")]
    pub free_space: String,
    pub threshold: String,
    #[tabled(rename = "used percentage")]
    pub used_percentage: String,
    #[tabled(rename = "mount point")]
    pub mount_point: String,
}


impl ResultStatus {
    pub fn from(self) -> Vec<MarkdownTable> {
        let Self { host, disk_status } = self;
        disk_status
            .into_iter()
            .map(|x| MarkdownTable {
                host: host.to_owned(),
                file_system: x.file_system,
                total_space: x.total_space,
                used_space: x.used_space,
                free_space: x.free_space,
                threshold: format!("{} %", x.target.threshold),
                used_percentage: icon_status(&x.used_percentage, x.target.threshold),
                mount_point: x.mount_point,
            })
            .collect()
    }
}

fn icon_status(val: &str, threshold: i8) -> String {
    match val.trim().parse::<i8>() {
        Ok(j) => {
            if j < threshold {
                format!("✅  {} %", j)
            } else {
                format!("⚠️  {} %", j)
            }
        },
        _ => val.into(),
    }
}

#[test]
fn test_icon_status() {
    assert_eq!(icon_status("0", 80), "✅  0 %");
    assert_eq!(icon_status("30", 80), "✅  30 %");
    assert_eq!(icon_status("79", 80), "✅  79 %");
    assert_eq!(icon_status("80", 80), "⚠️  80 %");
    assert_eq!(icon_status("100", 80), "⚠️  100 %");
    assert_eq!(icon_status("n/a", 80), "n/a");
    assert_eq!(icon_status("60", 60), "⚠️  60 %");
    assert_eq!(icon_status("0", 0), "⚠️  0 %");
}

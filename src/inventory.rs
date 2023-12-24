
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Inventory {
    pub server: Vec<Server>,
}
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub user_id: String,
    pub user_pw: String,
    pub observe_target: Vec<String>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct DiskStatus {
    pub file_system: String,
    pub total_space: String,
    pub used_space: String,
    pub free_space: String,
    pub used_percentage: String,
    pub mount_point: String,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ResultStatus {
    pub host: String,
    pub disk_status: Vec<DiskStatus>,
}

#[derive(Default, Debug, Clone, Tabled, Deserialize, Serialize)]
pub struct MarkdownTable {
    pub host: String,
    pub file_system: String,
    pub total_space: String,
    pub used_space: String,
    pub free_space: String,
    pub used_percentage: String,
    pub mount_point: String,
}

impl Inventory {
    pub fn set_server_info(&mut self, server: Server) {
        self.server.append(&mut vec![server]);
    }
}

impl ResultStatus {
    pub fn from(&self) -> Vec<MarkdownTable> {
        let host = &self.host.to_string();
        self.disk_status
            .iter()
            .map(|x| MarkdownTable {
                host: host.to_string(),
                file_system: x.file_system.to_string(),
                total_space: x.total_space.to_string(),
                used_space: x.used_space.to_string(),
                free_space: x.free_space.to_string(),
                used_percentage: ResultStatus::icon_status(String::from(x.used_percentage.to_string())),
                mount_point: x.mount_point.to_string(),
            }).collect()
    }

    fn icon_status(val: String) -> String {
        let i = val.trim().parse::<i32>();
        match i {
            Ok(j) => {
                match j {
                    i32::MIN..=79 => format!("✅  {} %", j),
                    80..=i32::MAX => format!("⚠️  {} %", j),
                }
            }
            _ => val
        }
    }
}

#[test]
fn test_icon_status() {
    assert_eq!(ResultStatus::icon_status(String::from("0")),"✅  0 %");
    assert_eq!(ResultStatus::icon_status(String::from("30")),"✅  30 %");
    assert_eq!(ResultStatus::icon_status(String::from("80")),"⚠️  80 %");
    assert_eq!(ResultStatus::icon_status(String::from("100")),"⚠️  100 %");
    assert_eq!(ResultStatus::icon_status(String::from("n/a")),"n/a");
}


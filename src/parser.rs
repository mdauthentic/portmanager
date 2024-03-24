use std::{
    io::Error,
    process::{Command, Stdio},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessDetail {
    pub process_name: String,
    pub pid: String,
    pub r#type: String,
    pub node: String,
    pub name: String,
}

pub fn list_process_unix() -> Result<Vec<u8>, Error> {
    let mut cmd = Command::new("lsof");
    cmd.args(["-i", "-P", "-n"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
        .map(|x| x.stdout)
}

pub fn kill_port(pid: String) -> Result<bool, Error> {
    let cmd = Command::new("kill")
        .args(["-9", pid.as_str()])
        .output()
        .expect("Unable to execute kill command");
    Ok(cmd.status.success())
}

pub fn parse_process() -> Vec<ProcessDetail> {
    let Ok(lsof_raw) = list_process_unix() else {
        return vec![];
    };
    let lsof_strings = String::from_utf8(lsof_raw).expect("failed to parse result to string");

    let lsof_each_line = lsof_strings
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    lsof_each_line
        .clone()
        .into_iter()
        .filter(|x| x.contains("(LISTEN)"))
        .map(|p| {
            let result = p.split_whitespace().collect::<Vec<_>>();
            let port = result[8].split(":").last().expect("unexpected error.");

            let details = ProcessDetail {
                process_name: result[0].to_string(),
                pid: result[1].to_string(),
                r#type: result[4].to_string(),
                node: result[7].to_string(),
                name: port.to_string(),
            };
            details
        })
        .collect::<Vec<_>>()
}

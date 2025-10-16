use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    sync::mpsc,
    thread,
    time::Duration,
};

use parse_size::parse_size;
use serde::{Deserialize, Serialize};

use crate::{parser, ComposeCommand, ComposeError};

use super::CatchOutput;

//{"BlockIO":"0B / 0B","CPUPerc":"0.03%","Container":"9ca40acb565a","ID":"9ca40acb565a","MemPerc":"0.13%","MemUsage":"10MiB / 7.685GiB","Name":"examples-rqlite-1","NetIO":"1.39kB / 0B","PIDs":"10"}

#[derive(Serialize, Debug, Clone)]
pub struct StatsUsage {
    usage: u64,
    limit: u64,
}

#[derive(Serialize, Debug, Clone)]
pub struct StatsIO {
    input: u64,
    output: u64,
}

// Implement the Deserialize trait for StatsIO, same as StatsUsage
impl<'de> serde::Deserialize<'de> for StatsIO {
    fn deserialize<D>(deserializer: D) -> Result<StatsIO, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StatsIOVisitor;

        impl<'de> serde::de::Visitor<'de> for StatsIOVisitor {
            type Value = StatsIO;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string in the format 'input/output'")
            }

            fn visit_str<E>(self, value: &str) -> Result<StatsIO, E>
            where
                E: serde::de::Error,
            {
                let parts = value.split('/').map(str::trim).collect::<Vec<&str>>();

                let input = parse_size(parts[0]).map_err(|_| E::custom("Failed to parse input"))?;
                let output =
                    parse_size(parts[1]).map_err(|_| E::custom("Failed to parse output"))?;
                Ok(StatsIO { input, output })
            }
        }

        deserializer.deserialize_str(StatsIOVisitor)
    }
}

impl<'de> serde::Deserialize<'de> for StatsUsage {
    fn deserialize<D>(deserializer: D) -> Result<StatsUsage, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StatsUsageVisitor;

        impl<'de> serde::de::Visitor<'de> for StatsUsageVisitor {
            type Value = StatsUsage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string in the format 'current/total'")
            }

            fn visit_str<E>(self, value: &str) -> Result<StatsUsage, E>
            where
                E: serde::de::Error,
            {
                let parts = value.split('/').map(str::trim).collect::<Vec<&str>>();

                let current =
                    parse_size(parts[0]).map_err(|_| E::custom("Failed to parse current"))?;
                let total = parse_size(parts[1]).map_err(|_| E::custom("Failed to parse total"))?;
                Ok(StatsUsage {
                    usage: current,
                    limit: total,
                })
            }
        }

        deserializer.deserialize_str(StatsUsageVisitor)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct StatsPercentage(f64);

impl<'de> serde::Deserialize<'de> for StatsPercentage {
    fn deserialize<D>(deserializer: D) -> Result<StatsPercentage, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StatsPercentageVisitor;

        impl<'de> serde::de::Visitor<'de> for StatsPercentageVisitor {
            type Value = StatsPercentage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string in the format '0.00%'")
            }

            fn visit_str<E>(self, value: &str) -> Result<StatsPercentage, E>
            where
                E: serde::de::Error,
            {
                let value = value.trim_end_matches('%');
                let value = value.parse::<f64>().map_err(E::custom)?;
                Ok(StatsPercentage(value))
            }
        }

        deserializer.deserialize_str(StatsPercentageVisitor)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Stats {
    #[serde(rename = "BlockIO")]
    pub block_io: StatsIO,
    #[serde(rename = "CPUPerc")]
    pub cpu_perc: StatsPercentage,
    pub container: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub mem_perc: StatsPercentage,
    pub mem_usage: StatsUsage,
    pub name: String,
    #[serde(rename = "NetIO")]
    pub net_io: StatsIO,
    #[serde(rename = "PIDs")]
    pub pids: String,
}

pub struct StatsCommand {
    command: std::process::Command,
    poll_interval: Option<Duration>,
}

type StatsIterator = Box<dyn Iterator<Item = Result<Vec<Stats>, ComposeError>> + Send>;

impl StatsCommand {
    pub fn new(cmd: std::process::Command) -> Self {
        Self {
            command: cmd,
            poll_interval: None,
        }
    }

    pub fn stream(self) -> Result<StatsIterator, ComposeError> {
        let mut command = self.command;

        command.arg("stats").arg("--format").arg("json");

        let stdout = command
            .stdout(std::process::Stdio::piped())
            .spawn()?
            .stdout
            .ok_or(ComposeError::IoError(std::io::Error::other(
                "Failed to open stdout",
            )))?;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let interval = self.poll_interval.unwrap_or(Duration::from_secs(1));

            loop {
                let mut lines = Vec::new();
                let start_time = std::time::Instant::now();

                while start_time.elapsed() < interval {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(0) => break,
                        Ok(_) => {
                            let line = parser::remove_ansi_codes(&line);

                            if let Ok(line) = line {
                                lines.push(line.trim().to_string());
                            } else {
                                tx.send(Err(ComposeError::ParseError(
                                    "Failed to parse line".to_string(),
                                )))
                                .expect("Failed to send error");
                                return;
                            }
                        }
                        Err(err) => {
                            tx.send(Err(ComposeError::IoError(err)))
                                .expect("Failed to send error");
                            return;
                        }
                    }
                }

                if lines.is_empty() {
                    break;
                }

                let stats = lines
                    .iter()
                    .map(|line| serde_json::from_str(line))
                    .collect::<Result<Vec<Stats>, _>>();

                match stats {
                    Ok(stats) => {
                        // if container occurs multiple times in the output, we calculate the average
                        let mut dedupe_stats_map: HashMap<String, Vec<Stats>> = HashMap::new();

                        for stat in stats {
                            let entry = dedupe_stats_map.entry(stat.container.clone()).or_default();
                            entry.push(stat);
                        }

                        let stats = dedupe_stats_map
                            .values()
                            .map(|stats| {
                                let mut avg_stats = stats[0].clone();
                                let len = stats.len() as u64;

                                for stat in stats.iter().skip(1) {
                                    avg_stats.block_io.input += stat.block_io.input;
                                    avg_stats.block_io.output += stat.block_io.output;
                                    avg_stats.cpu_perc.0 += stat.cpu_perc.0;
                                    avg_stats.mem_perc.0 += stat.mem_perc.0;
                                    avg_stats.mem_usage.usage += stat.mem_usage.usage;
                                    avg_stats.mem_usage.limit += stat.mem_usage.limit;
                                    avg_stats.net_io.input += stat.net_io.input;
                                    avg_stats.net_io.output += stat.net_io.output;
                                }

                                avg_stats.block_io.input /= len;
                                avg_stats.block_io.output /= len;
                                avg_stats.cpu_perc.0 /= len as f64;
                                avg_stats.mem_perc.0 /= len as f64;
                                avg_stats.mem_usage.usage /= len;
                                avg_stats.mem_usage.limit /= len;
                                avg_stats.net_io.input /= len;
                                avg_stats.net_io.output /= len;

                                avg_stats
                            })
                            .collect();

                        tx.send(Ok(stats)).expect("Failed to send stats");
                    }
                    Err(err) => {
                        tx.send(Err(ComposeError::ParseError(err.to_string())))
                            .expect("Failed to send error");
                        return;
                    }
                }
            }
        });

        Ok(Box::new(rx.into_iter()))
    }
}

impl ComposeCommand<Vec<Stats>, ()> for StatsCommand {
    const COMMAND: &'static str = "stats";

    fn exec(self) -> Result<Vec<Stats>, ComposeError> {
        let mut command = self.command;

        command
            .arg(Self::COMMAND)
            .arg("--format")
            .arg("json")
            .arg("--no-stream");

        let output = command.output().catch_output()?;

        let output = String::from_utf8_lossy(&output.stdout);

        let stats = output
            .lines()
            .map(serde_json::from_str)
            .collect::<Result<Vec<Stats>, _>>()?;

        Ok(stats)
    }
}

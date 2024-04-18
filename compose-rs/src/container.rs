use crate::ComposeError;

#[derive(Serialize, Debug)]
pub struct PortMapping {
    pub host_ip: Option<String>,
    pub host_port: Option<u16>,
    pub container_port: u16,
    pub protocol: String,
}

impl PortMapping {
    fn new(
        host_ip: Option<String>,
        host_port: Option<u16>,
        container_port: u16,
        protocol: String,
    ) -> Self {
        Self {
            host_ip,
            host_port,
            container_port,
            protocol,
        }
    }

    pub(crate) fn from_string(port_string: &str) -> Result<Vec<Self>, ComposeError> {
        // throw error
        let (ports, protocol) = port_string.split_at(port_string.find('/').ok_or(
            ComposeError::ParseError("Invalid port mapping: missing protocol".to_string()),
        )?);
        let protocol = protocol.trim_start_matches('/');

        // Some ports are not mapped to the host
        if !ports.contains("->") {
            // Also catch these cases: 4001-4002/tcp
            if ports.contains('-') {
                let (start, end) = ports.split_at(ports.find('-').ok_or(
                    ComposeError::ParseError("Invalid port mapping: missing '-'".to_string()),
                )?);
                let start = start.parse().map_err(|_| {
                    ComposeError::ParseError(format!(
                        "Invalid port mapping: invalid start port: {}",
                        start
                    ))
                })?;
                let end = end.trim_start_matches('-').parse().map_err(|_| {
                    ComposeError::ParseError(format!(
                        "Invalid port mapping: invalid end port: {}",
                        end
                    ))
                })?;

                return Ok((start..=end)
                    .map(|port| Self::new(None, None, port, protocol.to_string()))
                    .collect());
            }

            let container_port = ports.parse().map_err(|_| {
                ComposeError::ParseError("Invalid port mapping: invalid container port".to_string())
            })?;

            return Ok(vec![Self::new(
                None,
                None,
                container_port,
                protocol.to_string(),
            )]);
        }

        let (host, container_port) = ports.split_at(ports.find("->").ok_or(
            ComposeError::ParseError("Invalid port mapping: missing '->'".to_string()),
        )?);
        let container_port = container_port
            .trim_start_matches("->")
            .parse()
            .map_err(|_| {
                ComposeError::ParseError("Invalid port mapping: invalid container port".to_string())
            })?;

        let (host_ip, host_port) = host.split_at(host.find(':').ok_or(
            ComposeError::ParseError("Invalid port mapping: missing host port".to_string()),
        )?);

        let host_port = host_port.trim_start_matches(':').parse().map_err(|_| {
            ComposeError::ParseError("Invalid port mapping: invalid host port".to_string())
        })?;

        Ok(vec![Self::new(
            Some(host_ip.to_string()),
            Some(host_port),
            container_port,
            protocol.to_string(),
        )])
    }
}

#[derive(Serialize, Debug)]
pub enum Status {
    Up,
    Down,
    Restarting,
    Paused,
    Removing,
    Exited,
    Dead,
}

impl Status {
    fn from_string(status: &str) -> Self {
        match status.to_lowercase().as_str() {
            "up" => Self::Up,
            "down" => Self::Down,
            "restarting" => Self::Restarting,
            "paused" => Self::Paused,
            "removing" => Self::Removing,
            "exited" => Self::Exited,
            "dead" => Self::Dead,
            _ => panic!("Unknown status"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ContainerStatus {
    pub status: Status,
    pub since: String,
    pub exit_code: Option<u8>,
}

impl ContainerStatus {
    fn new(status: Status, since: String, exit_code: Option<u8>) -> Self {
        Self {
            status,
            since,
            exit_code,
        }
    }

    pub(crate) fn from_string(status: &str) -> Self {
        if status.contains(" (") {
            let parts: Vec<&str> = status.split(' ').collect();

            let status = Status::from_string(parts[0]);
            let exit_code = parts[1].trim_matches(|c| c == '(' || c == ')').parse().ok();
            let since = parts[2..].join(" ");

            return Self::new(status, since, exit_code);
        }

        let parts: Vec<&str> = status.split(' ').collect();
        let status = Status::from_string(parts[0]);
        let since = parts[1..].join(" ");

        Self::new(status, since, None)
    }
}

#[derive(Serialize, Debug)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub command: String,
    pub service: String,
    pub created: String,
    pub status: ContainerStatus,
    pub ports: Vec<PortMapping>,
}

use crate::{
    container::{Container, ContainerStatus, PortMapping},
    ComposeError,
};

pub(crate) fn parse_ps(output: &str) -> Result<Vec<Container>, ComposeError> {
    let mut lines = output.lines();

    let header = lines
        .next()
        .ok_or(ComposeError::ParseError("Missing header".to_string()))?;

    let header_indices = find_header_indices(header);
    let header_map: std::collections::HashMap<&str, usize> = header
        .split_whitespace()
        .enumerate()
        .map(|(i, h)| (h, i))
        .collect();

    let result = lines
        .map(|line| {
            let parts: Vec<&str> = header_indices
                .iter()
                .enumerate()
                .map(|(index, (_header_name, header_index))| {
                    // These *should* be safe to unwrap because we're guaranteed to have a match (?)
                    if let Some(next_index) = header_indices.get(index + 1) {
                        line.get(*header_index..next_index.1).unwrap().trim()
                    } else {
                        line.get(*header_index..).unwrap().trim()
                    }
                })
                .collect();

            Container {
                name: parts[header_map["NAME"]].to_string(),
                image: parts[header_map["IMAGE"]].to_string(),
                command: parts[header_map["COMMAND"]].trim_matches('"').to_string(),
                service: parts[header_map["SERVICE"]].to_string(),
                created: parts[header_map["CREATED"]].to_string(),
                status: ContainerStatus::from_string(parts[header_map["STATUS"]]),
                ports: parts[header_map["PORTS"]]
                    .split(", ")
                    .filter(|p| !p.is_empty())
                    .flat_map(|p| PortMapping::from_string(p).unwrap_or_default())
                    .collect(),
            }
        })
        .collect();

    Ok(result)
}

fn find_header_indices(header_line: &str) -> Vec<(&str, usize)> {
    let headers: Vec<&str> = header_line.split_whitespace().collect();

    headers
        .iter()
        // We can unwrap here because we know that the header exists in the line
        .map(|h| (*h, header_line.find(h).unwrap()))
        .collect()
}

pub(crate) fn remove_ansi_codes(input: &str) -> Result<String, ComposeError> {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*[mGKHfJ]")
        .map_err(|e| ComposeError::ParseError(format!("Failed to compile regex: {}", e)))?;

    Ok(re.replace_all(input, "").to_string())
}

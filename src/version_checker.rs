use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct BlenderVersion {
    pub raw_version_string: String,
    pub bit: u8,
    pub version: String,
}

impl BlenderVersion {
    pub fn to_raw_version(human_readable_version: &str) -> String {
        match &human_readable_version[0..1] {
            "1" | "2" => human_readable_version.replace(".", ""),
            _ => {
                let numbers: Vec<&str> = human_readable_version.split(".").collect();
                format!("{}.{:02}", numbers[0], numbers[1])
            },
        }
    }
    pub fn from_raw_version(raw_version: &str) -> String {
        match &raw_version[0..1] {
            "1" | "2" => format!("{}.{}", &raw_version[0..1], &raw_version[1..]),
            _ => format!("{}.{}", &raw_version[0..1], &raw_version[1..].parse::<u8>().unwrap()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InvalidVersionHeader {
    content: String,
}
impl fmt::Display for InvalidVersionHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid version header {}", self.content)
    }
}

pub fn get_version(version_header: &str) -> Result<BlenderVersion, InvalidVersionHeader> {
    let content = version_header.trim_end_matches(['H' /* 2~4 */, ' ' /* 1.80a */]);
    let prefix = "BLENDER";
    let suffix = "REND";
    if !(content.starts_with(prefix) && content.ends_with(suffix)) {
        return Err(InvalidVersionHeader { content: String::from(version_header) });
    }

    let raw_version_string = String::from(&content[prefix.len() + 2..content.len()-suffix.len()]);

    return Ok(BlenderVersion {
        version: BlenderVersion::from_raw_version(&raw_version_string),
        bit: if content.starts_with("BLENDER-") { 64 } else { 32 },
        raw_version_string,
    });
}

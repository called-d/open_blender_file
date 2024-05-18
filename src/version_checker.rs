use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct BlenderVersion {
    pub raw_version_string: String,
    pub bit: u8,
    pub version: String,
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
        raw_version_string: raw_version_string.clone(),
        bit: if content.starts_with("BLENDER-") { 64 } else { 32 },
        version: match &raw_version_string[0..1] {
            "1" | "2" => format!("{}.{}", &raw_version_string[0..1], &raw_version_string[1..]),
            _ => if raw_version_string.ends_with("0") {
                format!("{}.{}", &raw_version_string[0..1], &raw_version_string[1..2])
            } else {
                format!("{}.{}.{}", &raw_version_string[0..1], &raw_version_string[1..2], &raw_version_string[2..])
            },
        }
    });
}

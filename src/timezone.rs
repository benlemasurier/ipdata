#[derive(Deserialize, Debug)]
pub struct TimeZone {
    name: String,
    abbr: String,
    offset: String,
    // FIXME: the remote API will send an empty string if the value
    // is unknown. This breaks the serde_json type marshal.
    // is_dst: bool,
}

impl TimeZone {
    /// Returns the time zone's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the time zone's abbreviation, e.g., MST.
    pub fn abbr(&self) -> &String {
        &self.abbr
    }

    /// Returns UTC offset of the Timezone, e.g., "-0700".
    pub fn offset(&self) -> &String {
        &self.offset
    }
}

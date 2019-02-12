#[derive(Deserialize, Debug)]
pub struct Carrier {
    name: String,
    mcc: String,
    mnc: String,
}

impl Carrier {
    /// Returns the mobile carrier's name, if available.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the mobile carrier's country code, if available.
    pub fn mcc(&self) -> &String {
        &self.mcc
    }

    /// Returns the mobile carrier's network code, if available.
    pub fn mnc(&self) -> &String {
        &self.mnc
    }
}

#[derive(Deserialize, Debug)]
pub struct Language {
    name: String,
    native: String,
}

impl Language {
    /// Returns the IP location's language, in english.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the IP location's native language name.
    pub fn native(&self) -> &String {
        &self.native
    }
}

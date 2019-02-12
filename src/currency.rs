#[derive(Deserialize, Debug)]
pub struct Currency {
    name: String,
    code: String,
    symbol: String,
    native: String,
    plural: String,
}

impl Currency {
    /// Returns the name of the currency.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the ISO 4217 currency code.
    pub fn code(&self) -> &String {
        &self.code
    }

    /// Returns the currency's symbol.
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    /// Returns the native name of the currency.
    pub fn native(&self) -> &String {
        &self.native
    }

    /// Returns the plural version of the currency. For example, US dollars,
    /// Australian dollars, Euros.
    pub fn plural(&self) -> &String {
        &self.plural
    }
}

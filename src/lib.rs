#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;
use std::net;

mod error;
pub use error::IpDataError;

mod carrier;
pub use carrier::Carrier;

mod currency;
pub use currency::Currency;

mod language;
pub use language::Language;

mod timezone;
pub use timezone::TimeZone;

mod threat;
pub use threat::Threat;

/// Default URL for ipdata.co API. This can be configured with the
/// IPDATA_URL environment variable.
const DEFAULT_URL: &str = "https://api.ipdata.co";

#[derive(Deserialize, Debug)]
pub struct IpData {
    ip: String,
    is_eu: bool,
    city: String,
    region: String,
    region_code: String,
    country_name: String,
    country_code: String,
    continent_name: String,
    latitude: f64,
    longitude: f64,
    asn: String,
    organisation: String,
    postal: String,
    calling_code: String,
    flag: String,
    emoji_flag: String,
    emoji_unicode: String,
    languages: Vec<Language>,
    currency: Currency,
    time_zone: TimeZone,
    threat: Threat,
    count: String,
    carrier: Option<Carrier>,
}

impl IpData {
    /// Returns the IP address that was looked up.
    pub fn ip(&self) -> Result<net::IpAddr, net::AddrParseError> {
        self.ip.parse::<net::IpAddr>()
    }

    /// Returns true or false depending on whether the country is a recognized
    /// member of the European Union. The list of all EU countries is compiled
    /// from the European Union website:
    /// https://europa.eu/european-union/about-eu/countries_en.
    pub fn is_eu(&self) -> bool {
        self.is_eu
    }

    /// Returns the name of the city where the IP Address is located.
    pub fn city(&self) -> &String {
        &self.city
    }

    /// Returns the name of the region where the IP address is located.
    pub fn region(&self) -> &String {
        &self.region
    }

    /// Returns the ISO 3166-2 region code for the IP address.
    pub fn region_code(&self) -> &String {
        &self.region_code
    }

    /// Returns the country name where the IP address is located.
    pub fn country_name(&self) -> &String {
        &self.country_name
    }

    /// Returns the 2 letter ISO 3166-1 alpha-2 code for the country where the
    /// IP address is located.
    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    /// Returns the name of the continent where the IP Address is located. One
    /// of Africa, Antarctica, Asia, Europe, North America, Oceania, South America.
    pub fn continent_name(&self) -> &String {
        &self.continent_name
    }

    /// Returns an approximate latitudinal location for the IP Address. Often
    /// near the center of population.
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Returns an approximate longitudinal location for the IP Address. Often
    /// near the center of population.
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    /// Returns the Autonomous System Number that references the IP Address's
    /// owning organization.
    pub fn asn(&self) -> &String {
        &self.asn
    }

    /// Returns the name of the Organisation that owns the IP Address. This
    /// will default to the ISP name if the organisation is not available.
    pub fn organization(&self) -> &String {
        &self.organisation
    }

    /// Returns the postal code where the IP address is located.
    pub fn postal(&self) -> &String {
        &self.postal
    }

    /// Returns the international calling code where the IP adress is located.
    pub fn calling_code(&self) -> &String {
        &self.calling_code
    }

    /// Returns a URL to a PNG image with the flag of the country where the IP
    /// address is located.
    pub fn flag(&self) -> &String {
        &self.flag
    }

    /// Returns an emoji version of the flag of the country where the IP
    /// address is located.
    pub fn emoji_flag(&self) -> &String {
        &self.emoji_flag
    }

    /// Returns the country flag emoji in unicode.
    pub fn emoji_unicode(&self) -> &String {
        &self.emoji_unicode
    }

    /// Returns the location's languages.
    pub fn languages(&self) -> &Vec<Language> {
        &self.languages
    }

    /// Returns the location's local currency.
    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    /// Returns the locations local time zone.
    pub fn time_zone(&self) -> &TimeZone {
        &self.time_zone
    }

    /// Returns known threat data about the IP.
    pub fn threat(&self) -> &Threat {
        &self.threat
    }

    /// The total number of requests made by your API key in the last 24 hrs.
    /// Updates once a minute.
    pub fn count(&self) -> u64 {
        match self.count.parse::<u64>() {
            Ok(count) => count,
            Err(_) => 0, // FIXME: this could be better...
        }
    }

    /// Returns details of the IP Addresses's mobile carrier, if available.
    pub fn carrier(&self) -> &Option<Carrier> {
        &self.carrier
    }
}

#[derive(Deserialize, Debug)]
struct ResponseError {
    message: String,
}

/// Performs a lookup of the provided IpAddr.
///
/// # Examples
///
/// ```
/// # extern crate ipdata;
/// #
/// # use std::env;
/// # use std::net;
/// #
/// # fn main() {
///      let ip = net::Ipv4Addr::new(1,1,1,1);
///      let resp = ipdata::lookup(net::IpAddr::V4(ip));
///      println!("{}, {}", resp.latitude(), resp.longitude());
/// # }
/// ```
pub fn lookup(addr: net::IpAddr) -> Result<IpData, IpDataError> {
    let mut url = api_endpoint(addr)?;

    match api_key() {
        Some(key) => {
            url.set_query(Some(format!("api-key={}", key).as_str()));
        }
        None => eprintln!("warning: ipdata.co api key not configured"),
    }

    let resp = reqwest::get(url);
    let mut resp = match resp {
        Ok(resp) => resp,
        Err(err) => {
            return Err(request_error(err));
        }
    };

    if !resp.status().is_success() {
        match resp.json::<ResponseError>() {
            Ok(err) => return Err(IpDataError::new(err.message.as_str())),
            Err(_) => return Err(IpDataError::new("unknown failure occured")),
        }
    }

    match resp.json::<IpData>() {
        Ok(data) => Ok(data),
        Err(err) => {
            return Err(IpDataError::new(
                format!("failed to decode response: {}", err).as_str(),
            ));
        }
    }
}

fn request_error(err: reqwest::Error) -> IpDataError {
    let code = match err.status() {
        Some(code) => code,
        None => return IpDataError::new("unknown error"),
    };

    match code.canonical_reason() {
        Some(reason) => IpDataError::new(reason),
        None => IpDataError::new("unknown error"),
    }
}

fn api_endpoint(addr: net::IpAddr) -> Result<reqwest::Url, reqwest::UrlError> {
    let url = match env::var("IPDATA_URL") {
        Ok(url) => url,
        Err(_) => DEFAULT_URL.to_string(),
    };

    let url = reqwest::Url::parse(url.as_str())?;

    url.join(format!("{}", addr).as_str())
}

fn api_key() -> Option<String> {
    match env::var("IPDATA_KEY") {
        Ok(key) => Some(key),
        Err(_) => None,
    }
}

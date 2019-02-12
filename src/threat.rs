#[derive(Deserialize, Debug)]
pub struct Threat {
    is_tor: bool,
    is_proxy: bool,
    is_anonymous: bool,
    is_known_attacker: bool,
    is_known_abuser: bool,
    is_threat: bool,
    is_bogon: bool,
}

impl Threat {
    /// Returns true if the IP address s a known Tor exit node or relay.
    pub fn is_tor(&self) -> bool {
        self.is_tor
    }

    /// Returns true if the IP is a known proxy of any type.
    pub fn is_proxy(&self) -> bool {
        self.is_proxy
    }

    /// Returns true if is_tor or is_proxy is true
    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }

    /// Returns true if the IP is a known (reported) source
    /// of malicious activity.
    pub fn is_known_attacker(&self) -> bool {
        self.is_known_attacker
    }

    /// Returns true if the IP s a known (reported) source of abuse.
    pub fn is_known_abuser(&self) -> bool {
        self.is_known_abuser
    }

    /// Returns true if is_known_abuser or is_known_attacker is true.
    pub fn is_threat(&self) -> bool {
        self.is_threat
    }

    /// Returns true if the ip address is a Bogon, i.e.,
    /// an unassigned, unaddressable IP address.
    pub fn is_bogon(&self) -> bool {
        self.is_bogon
    }
}

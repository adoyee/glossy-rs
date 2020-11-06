#[derive(Debug)]
pub struct PortRule {
    pub domain6: String,
    pub domain4: String,
    pub port6: u16,
    pub port4: u16,
    pub reverse: bool,
}

#[derive(Debug)]
pub enum TrafficWay {
    FromV4,
    FromV6,
}

#[derive(Debug)]
pub struct Domain {
    pub domain6: String,
    pub domain4: String,
}

impl Domain {
    pub fn from_str(domain: String) -> Domain {
        Domain {
            domain6: domain.to_string(),
            domain4: domain.to_string(),
        }
    }

    pub fn from_str2(v6: String, v4: String) -> Domain {
        Domain {
            domain6: v6.to_string(),
            domain4: v4.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FQDN {
    pub subdomain: Option<String>,
    pub domain_name: String,
    pub tld: String
}

enum_from_primitive! {
    #[repr(u8)]
    #[derive(Clone, Debug)]
    pub enum FQDNParsingError {
        Missing = 0x0
    }
}

impl FQDN {
    pub fn new() -> FQDN {
        FQDN { 
            subdomain: None, 
            domain_name: String::from(""), 
            tld: String::from("") 
        }
    }

    pub fn len(&self) -> usize {
        if self.subdomain.is_none() {
            return self.domain_name.len() +
                self.tld.len();
        } else {
            return self.domain_name.len() +
                self.subdomain.as_ref().unwrap().len() +
                self.tld.len();
        }
    }

    pub fn split(&self) -> Vec<String> {
        if self.subdomain.is_none() {
            vec![
                self.domain_name.clone(),
                self.tld.clone()
            ] 
        } else {
            vec![
                self.subdomain.as_ref().unwrap().clone(),
                self.domain_name.clone(),
                self.tld.clone()
            ]
        }
    }
}

impl TryFrom<String> for FQDN {
    type Error = FQDNParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let splitted = value.split(".")
            .map(|item| {
                item.to_string()
            })
            .collect::<Vec<String>>();

        match splitted.len() {
            2 => {
                return Ok(
                    FQDN { 
                        subdomain: None, 
                        domain_name: splitted[0].clone(),
                        tld: splitted[1].clone()
                    }
                )
            },

            3 => {
                return Ok(
                    FQDN {
                        subdomain: Some(splitted[0].clone()),
                        domain_name: splitted[1].clone(),
                        tld: splitted[2].clone()
                    }
                )
            },

            _ => {
                return Err::<Self, Self::Error>(
                    FQDNParsingError::Missing
                );
            }
        }
    }
}
use serde::{Deserialize, Serialize};

use crate::forecast::Weather;

use super::Provider;
use super::ProviderError;

#[derive(Deserialize, Serialize, Debug)]
pub struct DummyProvider {
    pub is_valid: bool,
}

impl Provider for DummyProvider {
    fn current(&self, address_string: &str) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        if address_string.is_empty() {
            return Err(ProviderError::InvalidAddressFormat);
        }

        if address_string == "Paris,z" {
            return Err(ProviderError::InvalidCountryCode);
        }

        if address_string == "Paris,UU" {
            return Err(ProviderError::Unauthorized);
        }

        if address_string == "Paris,AA" {
            return Err(ProviderError::Unknown);
        }

        if address_string == "Paris,XX" {
            return Err(ProviderError::NoMatchingLocationFound);
        }

        if address_string == "Paris,FR" {
            return Ok(Weather { temperature: 10.22 });
        }

        Ok(Weather { temperature: 10.22 })
    }

    fn daily(&self, address_string: &str, _timestamp: i64) -> Result<Weather, ProviderError> {
        if !self.is_valid() {
            return Err(ProviderError::InvalidConfiguration);
        }

        if address_string.is_empty() {
            return Err(ProviderError::InvalidAddressFormat);
        }

        if address_string == "Paris,z" {
            return Err(ProviderError::InvalidCountryCode);
        }

        if address_string == "Paris,UU" {
            return Err(ProviderError::Unauthorized);
        }

        if address_string == "Paris,AA" {
            return Err(ProviderError::Unknown);
        }

        if address_string == "Paris,XX" {
            return Err(ProviderError::NoMatchingLocationFound);
        }

        if address_string == "Paris,YY" {
            return Err(ProviderError::MissingRequestedDate);
        }

        if address_string == "Paris,FR" {
            return Ok(Weather { temperature: 10.22 });
        }

        Ok(Weather { temperature: 10.22 })
    }

    fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl Default for DummyProvider {
    fn default() -> Self {
        Self { is_valid: true }
    }
}

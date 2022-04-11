use super::{Address, GeocodeError};

pub fn run(address_string: &str) -> Result<Address, GeocodeError> {
    let split = address_string.split(',');
    let vec: Vec<&str> = split.collect();
    let city: &str;
    let country_alpha_2_code: &str;

    if vec.len() == 2 {
        city = vec[0];
        country_alpha_2_code = vec[1];
    } else {
        return Err(GeocodeError::InvalidAddressFormat);
    }

    if country_alpha_2_code.len() != 2 {
        return Err(GeocodeError::InvalidCountryCode);
    }

    Ok(Address {
        city: city.to_string(),
        country_alpha_2_code: country_alpha_2_code.to_string(),
    })
}

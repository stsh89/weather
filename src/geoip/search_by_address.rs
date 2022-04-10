use super::Address;
use super::Point;
use super::GeoipError;
use super::Config;

pub fn run(config: Config, address: Address) -> Result<Point, GeoipError> {
    if !is_address_valid(&address) {
        return Err(GeoipError::NothingToGeocode);
    }

    let q = q(address);
    match config.client.search_by_address(q) {
        Ok(response) => Ok(Point { latitude: response.lat, longitude: response.lon }),
        Err(error) => Err(error),
    }
}

fn is_address_valid(address: &Address) -> bool {
    matches!((&address.city, &address.country_alpha_2_code), (Some(_), Some(_)))
}

fn q(address: Address) -> String {
    format!("{},{}", address.city.unwrap(), address.country_alpha_2_code.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geoip::DummyClient;

    #[test]
    fn it_returns_nothing_to_geocode_error() {
        let city = None;
        let country_alpha_2_code = None;
        let config = Config { client: Box::new(DummyClient {}) };
        let address = Address { city, country_alpha_2_code };
        let result = run(config, address);

        match result {
            Err(GeoipError::NothingToGeocode) => {},
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_unknown_geoip_error() {
        let city = Some("Paris".to_string());
        let country_alpha_2_code = Some("US".to_string());
        let config = Config { client: Box::new(DummyClient {}) };
        let address = Address { city, country_alpha_2_code };
        let result = run(config, address);

        match result {
            Err(GeoipError::Unknown) => {},
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_not_found_geoip_error() {
        let city = Some("Paris".to_string());
        let country_alpha_2_code = Some("ZZ".to_string());
        let config = Config { client: Box::new(DummyClient {}) };
        let address = Address { city, country_alpha_2_code };
        let result = run(config, address);

        match result {
            Err(GeoipError::NotFound) => {},
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_a_geoip_point() {
        let city = Some("Paris".to_string());
        let country_alpha_2_code = Some("FR".to_string());
        let config = Config { client: Box::new(DummyClient {}) };
        let address = Address { city, country_alpha_2_code };
        let result = run(config, address);

        match result {
            Ok(point) => {
                assert_eq!(point.latitude, 48.8588897);
                assert_eq!(point.longitude, 2.3200410217200766);
            }
            _ => unreachable!(),
        }
    }
}

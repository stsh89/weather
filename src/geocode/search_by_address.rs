use super::Address;
use super::Client;
use super::GeocodeError;
use super::Point;

pub fn run(client: &dyn Client, address: &Address) -> Result<Point, GeocodeError> {
    if !is_address_valid(address) {
        return Err(GeocodeError::NothingToGeocode);
    }

    let q = q(address);

    match client.search_by_address(q) {
        Ok(response) => Ok(Point {
            latitude: response.lat,
            longitude: response.lon,
        }),
        Err(error) => Err(error),
    }
}

fn is_address_valid(address: &Address) -> bool {
    !address.city.is_empty() && !address.country_alpha_2_code.is_empty()
}

fn q(address: &Address) -> String {
    format!("{},{}", address.city, address.country_alpha_2_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geocode::dummy_client::DummyClient;

    #[test]
    fn it_returns_nothing_to_geocode_error() {
        let city = "".to_string();
        let country_alpha_2_code = "".to_string();
        let client = DummyClient {};
        let address = Address {
            city,
            country_alpha_2_code,
        };
        let result = run(&client, &address);

        match result {
            Err(GeocodeError::NothingToGeocode) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_unknown_geocode_error() {
        let city = "Paris".to_string();
        let country_alpha_2_code = "US".to_string();
        let client = DummyClient {};
        let address = Address {
            city,
            country_alpha_2_code,
        };
        let result = run(&client, &address);

        match result {
            Err(GeocodeError::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_not_found_geocode_error() {
        let city = "Paris".to_string();
        let country_alpha_2_code = "ZZ".to_string();
        let client = DummyClient {};
        let address = Address {
            city,
            country_alpha_2_code,
        };
        let result = run(&client, &address);

        match result {
            Err(GeocodeError::NotFound) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_returns_a_geocode_point() {
        let city = "Paris".to_string();
        let country_alpha_2_code = "FR".to_string();
        let client = DummyClient {};
        let address = Address {
            city,
            country_alpha_2_code,
        };
        let result = run(&client, &address);

        match result {
            Ok(point) => {
                assert_eq!(point.latitude, 48.8588897);
                assert_eq!(point.longitude, 2.3200410217200766);
            }
            _ => unreachable!(),
        }
    }
}

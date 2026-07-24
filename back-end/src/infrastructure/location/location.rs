use maxminddb::{geoip2, Reader};
use std::net::IpAddr;


pub fn get_location(ip: IpAddr) -> Result<String, String> {
    let reader = Reader::open_readfile("./src/infrastructure/location/GeoLite2-City.mmdb").expect("Erro na leitura do reader para obtenção da localização");

    let result = reader.lookup(ip).expect("Erro na obtenção do result a partir do ip");

    let mut country : Option<&str> = None;

    if let Some(data) = result.decode::<geoip2::City>().expect("Erro no decode para obtenção da localização") {
        if data.country.iso_code.is_some() {
            country = Some(data.country.iso_code.unwrap());
        } else {
            country = Some(data.registered_country.iso_code.unwrap_or("N/A"));
        }
    }

    if country.is_some() {
        return Ok(country.unwrap().to_string());
    }

    Err(String::from("Localização não encontrada"))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_location_success_google() {
        let ip: IpAddr = "8.8.8.8".parse().expect("Erro na conversão do ip");

        let country = get_location(ip).expect("Erro ao obter localização a partir do ip");

        assert_eq!(country, String::from("US"));
    }

    #[test]
    fn test_get_location_success_cloudflare() {
        let ip: IpAddr = "1.1.1.1".parse().expect("Erro na conversão do ip");

        let country = get_location(ip).expect("Erro ao obter localização a partir do ip");

        assert_eq!(country, String::from("AU"));
    }

    #[test]
    fn test_get_location_success_quad9() {
        let ip: IpAddr = "9.9.9.9".parse().expect("Erro na conversão do ip");

        let country = get_location(ip).expect("Erro ao obter localização a partir do ip");

        assert_eq!(country, String::from("US"));
    }

    #[test]
    fn test_get_location_success_cisco() {
        let ip: IpAddr = "208.67.222.222".parse().expect("Erro na conversão do ip");

        let country = get_location(ip).expect("Erro ao obter localização a partir do ip");

        assert_eq!(country, String::from("US"));
    }

    #[test]
    fn test_get_location_error() {
        let ip: IpAddr = "0.0.0.0".parse().expect("Erro na conversão do ip");

        let result = get_location(ip);

        assert!(result.is_err());
    }
}
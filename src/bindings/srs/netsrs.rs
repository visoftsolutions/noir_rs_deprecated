use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, RANGE};

#[derive(Debug)]
pub struct NetSrs {
    pub data: Vec<u8>,
    pub g2_data: Vec<u8>,
    pub num_points: u32,
}

impl NetSrs {
    pub fn new(num_points: u32) -> Self {
        NetSrs {
            num_points,
            data: Self::download_g1_data(num_points),
            g2_data: Self::download_g2_data(),
        }
    }

    fn download_g1_data(num_points: u32) -> Vec<u8> {
        const G1_START: u32 = 28;
        let g1_end: u32 = G1_START + num_points * 64 - 1;

        let mut headers = HeaderMap::new();
        headers.insert(
            RANGE,
            format!("bytes={}-{}", G1_START, g1_end).parse().unwrap(),
        );

        let response = Client::new()
            .get(
                "https://aztec-ignition.s3.amazonaws.com/MAIN%20IGNITION/monomial/transcript00.dat",
            )
            .headers(headers)
            .send()
            .unwrap();

        response.bytes().unwrap().to_vec()
    }

    fn download_g2_data() -> Vec<u8> {
        const G2_START: usize = 28 + 5040001 * 64;
        const G2_END: usize = G2_START + 128 - 1;

        let mut headers = HeaderMap::new();
        headers.insert(
            RANGE,
            format!("bytes={}-{}", G2_START, G2_END).parse().unwrap(),
        );

        let response = Client::new()
            .get("https://aztec-ignition.s3.amazonaws.com/MAIN%20IGNITION/monomial/transcript00.dat")
            .headers(headers)
            .send()
            .unwrap();

        response.bytes().unwrap().to_vec()
    }
}

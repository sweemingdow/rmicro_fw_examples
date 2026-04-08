use fw_base::utils::rand;
use fw_crypto::aes::AesKeyDisplayType;
use fw_crypto::aes::gcm::AesGcm;
use fw_error::FwResult;
use proto_bin::auth_api::auth_info_provider_server::AuthInfoProvider;
use proto_bin::auth_api::{ParseTokenReq, ParseTokenResp};
use std::env;
use std::time::Duration;
use tonic::{Request, Response, Status};

pub struct AuthInfoProviderImpl {
    aes_gcm: AesGcm,
    should_wait: bool,
}

impl AuthInfoProviderImpl {
    pub fn new(decrypt_token_key: &str) -> Self {
        let aes_gcm = AesGcm::from_str(decrypt_token_key, AesKeyDisplayType::B64).unwrap();
        let should_wait = env::var("SHOULD_WAIT")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        Self {
            aes_gcm,
            should_wait,
        }
    }
}

#[tonic::async_trait]
impl AuthInfoProvider for AuthInfoProviderImpl {
    async fn parse_token(
        &self,
        request: Request<ParseTokenReq>,
    ) -> Result<Response<ParseTokenResp>, Status> {
        // 1-2毫秒
        if self.should_wait {
            // tokio::time::sleep(Duration::from_millis(rand::rand_range(1, 2))).await;
        }

        let req = request.into_inner();
        match self.aes_gcm.decrypt(&req.token, &req.nonce) {
            Ok(plain) => {
                let segments = plain.split("#").collect::<Vec<&str>>();
                let uid = segments.get(1).map(|v| v.to_string()).unwrap();
                let client_type = segments.get(3).map(|v| v.parse::<u32>().unwrap()).unwrap();
                let role = segments.get(5).map(|v| v.to_string()).unwrap();

                Ok(Response::new(ParseTokenResp {
                    uid,
                    client_type,
                    role,
                }))
            }
            Err(fe) => Err(Status::internal(fe.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use fw_base::my_utils::{rand, time};
    use fw_crypto::aes::AesKeyDisplayType;
    use fw_crypto::aes::gcm::AesGcm;
    use std::time::Duration;

    #[test]
    fn test_token_info() {
        let uid = "test_u2";
        let k = "VpQYmYyoY/HQ4XAP1fKvVKmc4kfiOAkVjKF7VHebgeA=";
        let client_type = 1;

        // {rand_str}#{uid}#{rand_str}#{client_type}#{ts}#{role}
        let ts = time::plus(time::dur_from_days(30));
        let origin_info = format!(
            "{}#{}#{}#{}#{}#{}",
            rand::rand_str(8),
            uid,
            rand::rand_str(8),
            client_type,
            ts,
            "admin"
        );

        println!("{}", origin_info);

        let ag = AesGcm::from_str(k, AesKeyDisplayType::B64).unwrap();
        let (cipher, nonce) = ag.encrypt(&origin_info).unwrap();

        println!("cipher={}, nonce={}", cipher, nonce);
    }
}

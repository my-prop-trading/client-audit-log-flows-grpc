use core::panic;

use encryption::aes::{AesEncryptedData, AesKey};
use my_logger::LogEventCtx;

pub fn _encrypt_optional_field(
    not_encrypted_field: Option<&mut String>,
    key: &AesKey,
) -> Option<String> {
    if let Some(non_encrypted_field) = not_encrypted_field {
        let encoded_data = key.encrypt(non_encrypted_field.as_bytes());
        return Some(encoded_data.as_base_64());
    }

    None
}

pub fn encrypt_field(not_encrypted_field: &mut String, key: &AesKey) -> String {
    let encoded_data = key.encrypt(not_encrypted_field.as_bytes());
    return encoded_data.as_base_64();
}

pub fn decrypt_field(
    client_id: &str,
    base_64_encrypted_content: &String,
    key: &AesKey,
) -> String {
    let encrypted_data = AesEncryptedData::from_base_64(base_64_encrypted_content);

    match encrypted_data {
        Ok(encrypted_data) => {
            return decode_field_from_vec(client_id, &encrypted_data, key).unwrap();
        }
        Err(err) => {
            my_logger::LOGGER.write_fatal_error(
                "Decoding dto field. Decoding from Base64 step",
                format!("Err: {:?}", err),
                LogEventCtx::new().add("clientId", client_id),
            );

            panic!("Decoding dto field. Decoding from Base64 step");
        }
    }
}

pub fn _decrypt_field_optional(
    client_id: &str,
    base_64_encrypted_content: Option<&String>,
    key: &AesKey,
) -> Option<String> {
    let base_64_encrypted_content = base_64_encrypted_content?;

    let encrypted_data = AesEncryptedData::from_base_64(base_64_encrypted_content);

    match encrypted_data {
        Ok(encrypted_data) => {
            return decode_field_from_vec(client_id, &encrypted_data, key);
        }
        Err(err) => {
            my_logger::LOGGER.write_fatal_error(
                "Decoding dto field. Decoding from Base64 step",
                format!("Err: {:?}", err),
                LogEventCtx::new().add("clientId", client_id),
            );

            return None;
        }
    }
}

fn decode_field_from_vec(
    client_id: &str,
    encoded_bytes: &AesEncryptedData,
    key: &AesKey,
) -> Option<String> {
    let decoded_string = key.decrypt(encoded_bytes);

    if let Err(err) = &decoded_string {
        my_logger::LOGGER.write_fatal_error(
            "Decoding from AES failed.",
            format!("Err: {:?}", err),
            LogEventCtx::new().add("clientId", client_id),
        );

        return None;
    }

    Some(decoded_string.unwrap().into_string())
}
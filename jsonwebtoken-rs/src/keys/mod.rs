/// ssh-keygen -t rsa -b 4096 -m PEM
pub const PRIVATE_KEY_1: &[u8] = include_bytes!("priv_key_1.pem");

/// openssl rsa -pubout -in priv_key_1.pem > pub_key_1.pem
pub const PUBLIC_KEY_1: &[u8] = include_bytes!("pub_key_1.pem");

/// ssh-keygen -t rsa -b 4096 -m PEM
pub const PRIVATE_KEY_2: &[u8] = include_bytes!("priv_key_2.pem");

/// openssl rsa -pubout -in priv_key_2.pem > pub_key_2.pem
pub const PUBLIC_KEY_2: &[u8] = include_bytes!("pub_key_2.pem");

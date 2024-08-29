use ssh_agent_client_rs::Client;
use ssh_key::{HashAlg, PublicKey, SshSig};
use std::{fs::read_to_string, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let sock = std::env::var("SSH_AUTH_SOCK")?;
    let sock_path = PathBuf::new().join(sock);
    let mut client = Client::connect(&sock_path)?;

    let id = PublicKey::from_openssh(&read_to_string("id_ecdsa.pub")?)?;

    const NAMESPACE: &str = "hello";
    const HASH_ALG: HashAlg = HashAlg::Sha256;

    const COUNT: usize = 10_000;
    let mut failures = 0;
    for i in 0..COUNT {
        println!("{} / {COUNT}", i + 1);
        let nonce: [u8; 32] = rand::random();
        let blob = ssh_key::SshSig::signed_data(NAMESPACE, HASH_ALG, &nonce)?;
        let Ok(sig) = client.sign(&id, &blob) else {
            failures += 1;
            println!("  fail!");
            continue;
        };
        let sig = SshSig::new(id.clone().into(), NAMESPACE, HASH_ALG, sig)?;
        id.verify(NAMESPACE, &nonce, &sig).unwrap();
    }
    println!("{failures} / {COUNT} failures");

    Ok(())
}

pub mod gitlab;

#[cfg(test)]
mod tests {
    // use crate::gitlab::mulit;
    // use crate::gitlab::A;
    use crate::gitlab::Client;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        println!("GET https://www.rust-lang.org");

        let mut res = reqwest::blocking::get("https://www.rust-lang.org/")?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:?}", res.headers());

        // copy the response body directly to stdout
        res.copy_to(&mut std::io::stdout())?;

        println!("\n\nDone.");
        Ok(())
    }

    #[test]
    fn it_works3() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new_basic_auth_client("caiwenhui".to_string(), "caiwenhui-password".to_string());
        println!("{:?}", client);
        Ok(())
    }
}
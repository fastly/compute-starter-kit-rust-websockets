use fastly::{Error, Request};

fn main() -> Result<(), Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    let mut req = Request::from_client();

    if let Some("websocket") = req.get_header_str("Upgrade") {
        req.handoff_websocket("backend")?;
        Ok(())
    } else {
        req.set_pass(true);
        req.send("backend")?.send_to_client();
        Ok(())
    }
}

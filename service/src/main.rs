use std::{net::SocketAddr, path::Path};

use abi::{reservation_service_server::ReservationServiceServer, Config};
use anyhow::{Ok, Result};
use reservation_service::RsvpService;

#[tokio::main]
async fn main() -> Result<()> {
    // we should first try to load RESERVE_CONFIG envar, then try "./reservation.yml", then try "~/.config/reservation.yml"
    // then try "/etc/reservation.yml"

    let filename = std::env::var("RESERVATION_CONFIG").unwrap_or_else(|_| {
        let p1 = Path::new("./reservation.yml");
        let path = shellexpand::tilde("~/.config/reservation.yml");
        let p2 = Path::new(path.as_ref());
        let p3 = Path::new("/etc/reservation.yml");

        match (p1.exists(), p2.exists(), p3.exists()) {
            (true, _, _) => p1.to_str().unwrap().to_string(),
            (_, true, _) => p2.to_str().unwrap().to_string(),
            (_, _, true) => p3.to_str().unwrap().to_string(),
            _ => panic!("no config file found"),
        }
    });

    let config = Config::load(filename.as_str())?;
    println!("{config:?}");

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port).parse()?;

    let svc = RsvpService::from_config(&config).await;
    let svc = ReservationServiceServer::new(svc);

    println!("Listening on {addr}");
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}

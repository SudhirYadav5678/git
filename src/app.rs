use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;

use anyhow::Ok;

pub struct AuthJwt;

impl AuthJwt {
    pub async fn run() -> anyhow::Result<()> {
        let port = 3000;
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
        let app = crate::routes::root();
        let listner = TcpListener::bind(addr).await?;
        axum::serve(listner, app).await?;

        Ok(())
    }
}

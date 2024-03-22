use async_trait::async_trait;
use pingora::prelude::*;
use pingora_proxy::ProxyHttp;
use std::{env, sync::Arc};

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    /// For this small example, we don't need context storage
    type CTX = ();
    fn new_ctx(&self) {}

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self
            .0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        println!("upstream peer is: {upstream:?}");

        let peer = Box::new(HttpPeer::new(upstream, false, "0.0.0.0".to_string()));
        Ok(peer)
    }
}

fn main() {
    dotenvy::dotenv().unwrap();

    let mut my_server = Server::new(Some(Opt::default())).unwrap();
    my_server.bootstrap();

    // Note that upstreams needs to be declared as `mut` now
    let mut upstreams = LoadBalancer::try_from_iter(["0.0.0.0:8081", "0.0.0.0:8082"]).unwrap();

    let hc = TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(std::time::Duration::from_secs(1));

    let background = background_service("health check", upstreams);
    let upstreams = background.task();

    // `upstreams` no longer need to be wrapped in an arc
    let mut lb = http_proxy_service(&my_server.configuration, LB(upstreams));

    let port = env::var("PROXY_PORT").unwrap();
    lb.add_tcp(&format!("0.0.0.0:{port}"));

    my_server.add_service(background);

    my_server.add_service(lb);
    my_server.run_forever();
}

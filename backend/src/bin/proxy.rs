use std::env;

fn main() {
    dotenvy::dotenv().unwrap();
    let server_port = env::var("SERVER_PORT").unwrap();

    let mut upstreams = LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443"]).unwrap();

    let mut lb = pingora_proxy::http_proxy_service(&my_server.configuration, LB(upstreams));
    lb.add_tcp("127.0.0.1:6188");

    let mut my_server = Server::new(None).unwrap();
    my_server.add_service(lb);
    my_server.run_forever();
}

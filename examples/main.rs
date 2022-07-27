use rmw_config::config;

fn main() {
  config!(rmw);
  let mtu = get!(v4_mtu, 1440);
  dbg!(mtu);
}

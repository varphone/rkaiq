use rkaiq::sysctl::StaticMetas;

fn main() {
    let metas = StaticMetas::new();
    for m in metas {
        println!("Found sensor: {}\n{:#?}", m.sensor_name(), m);
    }
}

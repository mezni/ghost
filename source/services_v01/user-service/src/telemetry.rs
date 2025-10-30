pub fn init_telemetry(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    shared::telemetry::init_telemetry(service_name)
}

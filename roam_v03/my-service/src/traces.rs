use crate::errors::AppError;
use opentelemetry::{
    global,
    sdk::{
        trace::{self, RandomIdGenerator, Sampler, TracerProvider},
        Resource,
    },
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use std::time::Duration;

pub fn init_tracer() -> Result<trace::Tracer, AppError> {
    // Configure OTLP exporter
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:4317")
        .with_timeout(Duration::from_secs(3));

    // Create tracer provider
    let tracer_provider = TracerProvider::builder()
        .with_batch_exporter(
            exporter,
            opentelemetry::runtime::Tokio,
        )
        .with_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", "my-actix-service"),
                    KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                ])),
        )
        .build();

    // Set as global provider
    global::set_tracer_provider(tracer_provider);

    // Use W3C TraceContext propagator
    global::set_text_map_propagator(
        opentelemetry::sdk::propagation::TraceContextPropagator::new(),
    );

    // Create and return tracer
    Ok(global::tracer("actix-web"))
}
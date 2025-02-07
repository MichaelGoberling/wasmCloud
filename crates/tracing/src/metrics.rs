use anyhow::Context;
use heck::ToKebabCase;

#[cfg(not(feature = "otel"))]
pub fn configure_metrics(_: String, _: &wasmcloud_core::OtelConfig) -> anyhow::Result<()> {
    // no-op for when otel isn't enabled.
    Ok(())
}

#[cfg(feature = "otel")]
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::module_name_repetitions)]
pub fn configure_metrics(
    service_name: &str,
    otel_config: &wasmcloud_core::OtelConfig,
) -> anyhow::Result<()> {
    use opentelemetry_otlp::WithExportConfig;
    let normalized_service_name = service_name.to_kebab_case();

    let mut exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_protocol(opentelemetry_otlp::Protocol::HttpBinary);

    if let Some(ref endpoint) = otel_config.exporter_otlp_endpoint.clone() {
        exporter = exporter.with_endpoint(endpoint);
    }

    opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry_sdk::runtime::Tokio)
        .with_exporter(exporter)
        .with_resource(opentelemetry_sdk::Resource::new(vec![
            opentelemetry::KeyValue::new("service.name", normalized_service_name),
        ]))
        .with_aggregation_selector(ExponentialHistogramAggregationSelector::new())
        .with_temporality_selector(
            opentelemetry_sdk::metrics::reader::DefaultTemporalitySelector::new(),
        )
        .build()
        .context("failed to create OTEL metrics provider")?;

    Ok(())
}

/// Replaces the default `ExplicitBucketHistogram` aggregation for Histograms
/// with `Base2ExponentialHistogram`. This makes it easier to capture latency
/// at nanosecond accuracy.
#[derive(Clone, Default, Debug)]
struct ExponentialHistogramAggregationSelector {
    pub(crate) _private: (),
}

impl ExponentialHistogramAggregationSelector {
    /// Create a new  aggregation selector.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(feature = "otel")]
impl opentelemetry_sdk::metrics::reader::AggregationSelector
    for ExponentialHistogramAggregationSelector
{
    fn aggregation(
        &self,
        kind: opentelemetry_sdk::metrics::InstrumentKind,
    ) -> opentelemetry_sdk::metrics::Aggregation {
        match kind {
            opentelemetry_sdk::metrics::InstrumentKind::Counter
            | opentelemetry_sdk::metrics::InstrumentKind::UpDownCounter
            | opentelemetry_sdk::metrics::InstrumentKind::ObservableCounter
            | opentelemetry_sdk::metrics::InstrumentKind::ObservableUpDownCounter => {
                opentelemetry_sdk::metrics::Aggregation::Sum
            }
            opentelemetry_sdk::metrics::InstrumentKind::ObservableGauge => {
                opentelemetry_sdk::metrics::Aggregation::LastValue
            }
            opentelemetry_sdk::metrics::InstrumentKind::Histogram => {
                opentelemetry_sdk::metrics::Aggregation::Base2ExponentialHistogram {
                    max_size: 160,
                    max_scale: 20,
                    record_min_max: true,
                }
            }
        }
    }
}

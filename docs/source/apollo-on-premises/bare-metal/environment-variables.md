## Apollo

| Variable          | Description                                                                                     |
| ----------------- | ----------------------------------------------------------------------------------------------- |
| `APOLLO_TEMP_DIR` | Override the default temporary directory Apollo uses for scratch files and some I/O operations. |

You may also set any Apollo OSS-recognized environment variables.

## Apollo on-premises

| Variable                   | Description                                                                                                                                                                                                     |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `OTLP_ENDPOINT`            | Target endpoint for sending OTLP traces/metrics/logs to your OpenTelemetry collector/observability stack.<br>e.g. `http://otel-collector:4317`.                                                                 |
| `OTEL_SERVICE_INSTANCE_ID` | OpenTelemetry `service.instance.id` that uniquely identifies this node in telemetry.<br>Must match `instance_id`.                                                                                               |
| `PLC_LOG_LEVEL`            | Controls logging verbosity for the Apollo on-premises components (e.g. scheduler/worker).<br>e.g. `Info`, `Debug`, `Trace`, _etc._ (follows the Rust [naming](https://docs.rs/log/latest/log/enum.Level.html)). |

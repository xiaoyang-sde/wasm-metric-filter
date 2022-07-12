# WebAssembly Metric Filter

The WebAssembly Metric Filter is an Envoy filter developed with [proxy-wasm-rust-sdk](https://github.com/proxy-wasm/proxy-wasm-rust-sdk). The filter augments the HTTP streams and exports additional metrics that could improve monitoring systems. The metrics are rooted at `wasmcustom` and contain the following statistics:

|Name|Type|Description|Example|
|-|-|-|-|
|`upstream_rq_<status>_[<response_code_details>]`|Counter|The count of [response code details](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_conn_man/response_code_details) for each HTTP response status|`upstream_rq_500_[via_upstream]`|

## Install and Build

```shell
# Install wasm-pack
$ cargo install wasm-pack

# Compile from Rust to WebAssembly
# Build artifact: pkg/wasm_metric_filter_bg.wasm
$ wasm-pack build
```

## Example

```shell
# Create a reverse proxy for http://httpbin.org
$ envoy -c example/envoy_config.yaml

# Send several GET requests
$ curl localhost:8080/status/200
$ curl localhost:8080/status/400
$ curl localhost:8080/status/500

# Inspect the metrics exported from the filter
$ curl -s localhost:9901/stats | grep wasmcustom
wasmcustom.upstream_rq_200_[via_upstream]: 1
wasmcustom.upstream_rq_400_[via_upstream]: 1
wasmcustom.upstream_rq_500_[via_upstream]: 1
```

## Configuration

```yaml
http_filters:
  - name: envoy.filters.http.wasm
    typed_config:
      "@type": type.googleapis.com/udpa.type.v1.TypedStruct
      type_url: type.googleapis.com/envoy.extensions.filters.http.wasm.v3.Wasm
      value:
        config:
          configuration:
              "@type": type.googleapis.com/google.protobuf.StringValue
              # The filter configuration declares if a metric should be exported
              value: |
                {
                  "response_code_details": true,
                }
          vm_config:
            runtime: "envoy.wasm.runtime.v8"
            code:
              local:
                filename: "./pkg/wasm_metric_filter_bg.wasm"
```

## Reference

- [WebAssembly in Envoy](https://github.com/proxy-wasm/spec/blob/master/docs/WebAssembly-in-Envoy.md)
- [Proxy-Wasm vNEXT ABI specification](https://github.com/proxy-wasm/spec/blob/master/abi-versions/vNEXT/README.md)

fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(
            &[
                "../cloudapi/third_party/googleapis/google/api/http.proto",
                "../cloudapi/third_party/googleapis/google/api/annotations.proto",
                "../cloudapi/third_party/googleapis/google/rpc/status.proto",
            ],
            &["../cloudapi/third_party/googleapis/"],
        )
        .unwrap();
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(
            &[
                "../cloudapi/yandex/cloud/api/operation.proto",
                "../cloudapi/yandex/cloud/operation/operation.proto",
                "../cloudapi/yandex/cloud/ai/tts/v3/tts_service.proto",
                "../cloudapi/yandex/cloud/ai/tts/v3/tts.proto",
            ],
            &["../cloudapi/", "../cloudapi/third_party/googleapis/"],
        )
        .unwrap();
}

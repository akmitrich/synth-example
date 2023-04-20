use tokio_stream::StreamExt;

pub mod tts {
    tonic::include_proto!("speechkit.tts.v3");
}

#[tokio::main]
async fn main() {
    let pem = std::fs::read("/etc/ssl/certs/GlobalSign_Root_CA.pem").expect("read the cert file");
    let cert = tonic::transport::Certificate::from_pem(pem);
    let tls = tonic::transport::ClientTlsConfig::new().ca_certificate(cert);
    synthesize("Привет, Яндекс!", include_str!(".secret"), tls).await;
}

async fn synthesize(text: &str, token: &str, tls: tonic::transport::ClientTlsConfig) {
    let mut req = tonic::Request::new(tts::UtteranceSynthesisRequest {
        utterance: Some(tts::utterance_synthesis_request::Utterance::Text(
            text.into(),
        )),
        output_audio_spec: Some(tts::AudioFormatOptions {
            audio_format: Some(tts::audio_format_options::AudioFormat::ContainerAudio(
                tts::ContainerAudio {
                    container_audio_type: tts::container_audio::ContainerAudioType::Wav as _,
                },
            )),
        }),
        loudness_normalization_type:
            tts::utterance_synthesis_request::LoudnessNormalizationType::Lufs as _,
        ..Default::default()
    });
    req.metadata_mut().insert(
        "authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    let channel = tonic::transport::Channel::from_static("http://tts.api.cloud.yandex.net:443")
        .tls_config(tls)
        .expect("TLS config")
        .connect()
        .await
        .expect("tonic Channel");
    let mut stub = tts::synthesizer_client::SynthesizerClient::new(channel);
    match stub.utterance_synthesis(req).await {
        Ok(resp) => {
            println!(
                "Meta data keys: {:?}",
                resp.metadata().keys().collect::<Vec<_>>()
            );
            let mut resp = resp.into_inner();
            while let Some(chunk) = resp.next().await {
                match chunk {
                    Ok(resp) => match resp.audio_chunk {
                        Some(chunk) => println!("Audio chunk arrived. {} bytes.", chunk.data.len()),
                        None => println!("Audio chunk is empty."),
                    },
                    Err(status) => println!("Error: {}", status),
                }
            }
        }
        Err(status) => println!("Error in utterance_synthesis: {}", status),
    }
}

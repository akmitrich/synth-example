# Пример использования API v3 синтеза на Rust
По мотивам https://cloud.yandex.ru/docs/speechkit/tts/api/tts-examples-v3

## TLS-соединение
В tonic для открытия безопасного соединения нужно явно указать root certificate.

## Нужно склонировать git clone https://github.com/yandex-cloud/cloudapi
Поддиректория synth-example должна располагаться рядом с cloudapi. т.е. обе поддиректории должны находиться в общей директории. 

## В папке src 
Должны находится файлы .secret и .folder-id. В файл .secret надо записать IAM-token, а в файл .folder-id соответствующий folder-id.

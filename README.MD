# nofication-service

## Tecnologías:

- [Rust](https://www.rust-lang.org/)
- [Cargo (Package manager)](https://doc.rust-lang.org/cargo/)
- [Rocket (WEB API)](https://rocket.rs/)

## Prerequisitos:

- cargo 1.71.0-nightly or up / Docker

## Swagger

WIP

## Ejecución del proyecto (sin docker)

```
cargo run --package notifications-service --bin notifications-service
```

Levanta el servicio en el puerto 8000

```
   Compiling notifications-service v0.1.0 (/Users/lmatwiejczuk/git_repos/notifications-service)
    Finished dev [unoptimized + debuginfo] target(s) in 6.78s
     Running `target/debug/notifications-service`
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 4
   >> max blocking threads: 512
   >> ident: Rocket
   >> IP header: X-Real-IP
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> temp dir: /var/folders/hj/p2j2b3w97p7f6dtsnwxv_ntw0000gn/T/
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
📬 Routes:
   >> (ping) GET /ping
   >> (notification) POST /notification
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> X-Frame-Options: SAMEORIGIN
   >> Permissions-Policy: interest-cohort=()
   >> X-Content-Type-Options: nosniff
🚀 Rocket has launched from http://127.0.0.1:8000
```

## Inicialización y ejecución del proyecto (docker)

### Pasos:

1) Ir a la carpeta root del repositorio

2) Construir el Dockerfile (imagen) del servicio

```
docker build . -t notification-service
```

3) Ejecutar la imagen construida



Nota: Pedir credenciales por privado.


```
docker run -p <port>:8081 --env-file ./resources/local.env --name users-service users-service
```

Nota: agregar "-d" si se quiere ejecutar como deamon

```
docker run -d -p <port>:8081 --env-file ./resources/local.env --name users-service users-service
```

Ejemplo:

```
docker run -d -p 8081:8081 --env-file ./resources/local.env --name users-service users-service
```

4) En un browser, abrir swagger del servicio en el siguiente url:

`http://localhost:<port>/docs/index.html`

Segun el ejemplo:

`http://localhost:8081/docs/index.html`

5) Probar el endpoint health check y debe retornar ok

6) La API esta disponible para ser utilizada


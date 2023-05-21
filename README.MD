# notifications-service

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
docker build . -t notifications-service
```

3) Ejecutar la imagen construida

Se necesita configurar users-service en la red.

Nota: Pedir contraseña cuenta SMTP por privado.


```
docker run -p 8000:8000 --env-file ./resources/local.env --name notifications-service notifications-service
```

5) Probar el endpoint health check y debe retornar ok

```
curl "http://localhost:8000/ping" 
```


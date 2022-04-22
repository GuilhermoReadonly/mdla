# MDLA

## Quick start:

### Start Server
> cargo run --bin=mdla-server

### Start front
> trunk serve ./mdla-front/index.html --proxy-backend http://localhost:8000/api/

### Open brower
http://localhost:8080/


## Docker

### Build
> sudo docker build -t mdla .

### Run
> sudo docker run -ti -p 8000:8000 mdla
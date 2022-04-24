# MDLA

## Quick start:

### Start Server
```bash
cargo run --bin=mdla-server
```

### Start front
```bash
trunk serve ./mdla-front/index.html --proxy-backend http://localhost:8000/api/
```

### Open brower
http://localhost:8080/


## Docker

### Build
```bash
sudo docker build -t mdla .
```

### Run
```bash
sudo docker run -ti -p 8000:8000 mdla
```


## Setup dev env
Create a simlink from the dist folder of the webapp to the web-app folder of the server
```bash
mkdir resources
cd resources
ln -s ../mdla-front/dist/ web-app
```


## Tips

### Format your code
```bash
cargo fmt
```

### Improve your code

```bash
cargo clippy --fix
```
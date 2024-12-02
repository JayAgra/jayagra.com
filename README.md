# jayagra.com

use caddy
```sh
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https curl
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update
sudo apt install caddy
```

caddy file loc:<br>
`caddy run --config`

<!-- ## ssl
```sh
openssl req -x509 -newkey rsa:4096 -nodes -keyout ./ssl/key.pem -out ./ssl/cert.pem -days 365 -subj '/CN=localhost'
```

```sh
# new certificate. run commands from ~/bearTracks
certbot certonly --standalone --keep-until-expiring --agree-tos -d "<DOMAIN>"
cp /etc/letsencrypt/live/<DOMAIN>/cert.pem ssl/cert.pem
cp /etc/letsencrypt/live/<DOMAIN>/privkey.pem ssl/key.pem
# renew certificate. run from ~/bearTracks
certbot renew
cp /etc/letsencrypt/live/<DOMAIN>/cert.pem ssl/cert.pem
cp /etc/letsencrypt/live/<DOMAIN>/privkey.pem ssl/key.pem
``` -->
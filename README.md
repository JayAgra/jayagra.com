# jayagra.com

use caddy
```sh
# install
sudo apt install caddy
# config
sudo nano /etc/caddy/Caddyfile
# status
systemctl status caddy
# reload new config
sudo systemctl reload caddy
# stop
sudo systemctl stop caddy
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
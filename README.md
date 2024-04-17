# jayagra.com

## ssl
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
```
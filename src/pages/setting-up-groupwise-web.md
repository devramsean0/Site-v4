# What is this?
This is a very quick and dirty guide to installing Groupwise Web for Groupwise 24.4 in an environment where:

- We are using Self-Signed certificates
- It is not publicly exposed

# Step 1: Setup Docker
Just run these commands:

```
sudo zypper install docker
sudo systemctl enable docker
sudo systemctl start docker
docker run hello-world
```

# Step 2: Configuring SSL Certs
You need to generate some SSL certificates, The commands that worked for me are:

```
sudo mkdir -p /opt/novell/gw/certs
cd /opt/novell/gw/certs

sudo openssl req -newkey rsa:2048 -nodes -keyout server.key -x509 -days 365 -out server.crt -subj "/CN=<YOUR FQDN>" -addext "subjectAltName=DNS:<YOUR FQDN>"

```

Subsituting `<YOUR FQDN>` with the servers hostname

# Step 3: Run the Configuration container
This will produce some errors, but that is fine.

```
sudo docker run -it -v /optovell/gw:/config -e GWADMIN_SERVICE=admin@<ADMIN SERVER IP>:9710 -e GWSOAP_HOST_DEFAULT=<YOUR POA IP> mfgroupwise/web-config
```

You will be asked for your admin password, and if you want to verify the DVAs, I chose not to (not sure if it would work though)

# Step 4: Fix any references to your FQDN
This is only needed if your server is not exposed to the internet/on a DNS server. You could probably bypass this by passing in a /etc/host file, but this worked for me.

Modify the references to your FQDN to your IP in `/opt/novell/gw/web.conf` and `/opt/novell/gw/dvas.conf`

# Step 5: Run the actual container

```
sudo docker run --rm -d -v /opt/novell/gw:/etc/nginx/gw -e FQDN=<YOUR FQDN> -e DNS_SERVER=<A DNS SERVER LIKE 8.8.8.8> -p 80:80 -p 443:443 -v  /opt/novell/gw/certs:/certs -v /opt/novell/gw/logs:/var/log/nginx -e GWSOAP_SSL_VERIFY=off mfgroupwise/web:latest

```
You could probably not disable GWSOAP SSL Verification. But I once again have not tried this.

If you want to change the port numbers that the container will expose as, change `-p 80:80 -p 443:443` in the above command to `-p <Your wanted http port>:80 -p <Your wanted https port:443`. For example `-p 880:80 -p 8443:443` will make http avaliable on port 880 and https avaliable on port 8443




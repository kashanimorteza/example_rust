<!--------------------------------------------------------------------------------- Description -->
# example_rust
    this is example of rust

<!--------------------------------------------------------------------------------- Resource -->
## Resource  
    Web    : https://radkesvat.github.io/WaterWall-Docs/
    Github : https://github.com/radkesvat/WaterWall

<!--------------------------------------------------------------------------------- Installation -->
## Installation  
```bash
git clone git@github.com:kashanimorteza/example_waterwall.git  
cd ./example_waterwall
```

<!--------------------------------------------------------------------------------- Transfer -->
## Transfer

```bash
rsync -avz --exclude Waterwall /Volumes/data/projects/example_waterwall/* root@sh1.mk1001.site:/root/example_waterwall/
rsync -avz --exclude Waterwall /Volumes/data/projects/example_waterwall/* root@sh2.mk1001.site:/root/example_waterwall/
rsync -avz --exclude Waterwall /Volumes/data/projects/example_waterwall/* root@si1.mk1001.site:/root/example_waterwall/
rsync -avz --exclude Waterwall /Volumes/data/projects/example_waterwall/* root@si2.mk1001.site:/root/example_waterwall/
rsync -avz --exclude Waterwall /Volumes/data/projects/example_waterwall/* root@si3.mk1001.site:/root/example_waterwall/
rsync -avz --exclude Waterwall /Volumes/data/projects/example_waterwall/* root@si4.mk1001.site:/root/example_waterwall/
```

<!--------------------------------------------------------------------------------- Configure -->
## Configure

```bash
/root/example_waterwall/cli.sh remove
/root/example_waterwall/cli.sh create_folder
/root/example_waterwall/cli.sh create_config "$(hostname)"  general
/root/example_waterwall/cli.sh create_config "$(hostname)"  old
/root/example_waterwall/cli.sh create_config "$(hostname)"  new
/root/example_waterwall/cli.sh stop
/root/example_waterwall/cli.sh start
/root/example_waterwall/cli.sh enable
```

<!--------------------------------------------------------------------------------- TlS Tunnel -->
## TlS Tunnel

#### Create certificate
```bash
sudo certbot certonly --standalone --preferred-challenges http --agree-tos --email mkvpn1001@gmail.com -d cert1.irankohans.info
sudo certbot certonly --standalone --preferred-challenges http --agree-tos --email mkvpn1001@gmail.com -d waterwall_si3.irankohans.info
/etc/letsencrypt/live/waterwall.irankohans.info/fullchain.pem
/etc/letsencrypt/live/waterwall.irankohans.info/privkey.pem
```

#### Download certificate
```bash
rsync -avz root@sh1.mk1001.site:/etc/letsencrypt/archive/waterwall.irankohans.info/cert1.pem ./cert.pem
rsync -avz root@sh1.mk1001.site:/etc/letsencrypt/archive/waterwall.irankohans.info/chain1.pem ./chain.pem
rsync -avz root@sh1.mk1001.site:/etc/letsencrypt/archive/waterwall.irankohans.info/fullchain1.pem ./fullchain.pem
rsync -avz root@sh1.mk1001.site:/etc/letsencrypt/archive/waterwall.irankohans.info/privkey1.pem ./privkey.pem
```
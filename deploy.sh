#!/usr/bin/env zsh

cargo build --release
sudo systemctl stop edmondburnett-com.service
#sudo mkdir -p /srv/http/edmondburnett.com
#sudo useradd --system --no-create-home --shell /usr/bin/nologin edmondburnett-com
sudo cp target/release/edmondburnett-com /srv/http/edmondburnett.com
sudo cp -r pages /srv/http/edmondburnett.com
sudo cp -r posts /srv/http/edmondburnett.com
sudo cp -r projects /srv/http/edmondburnett.com
sudo cp -r static /srv/http/edmondburnett.com
sudo chown -R edmondburnett-com:edmondburnett-com /srv/http/edmondburnett.com
sudo systemctl link /srv/http/edmondburnett.com/edmondburnett-com.service
sudo systemctl daemon-reload
sudo systemctl restart edmondburnett-com.service

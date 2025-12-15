.PHONY: install

install:
	cargo build --release
	sudo rm -f /usr/bin/dokku-daemon /etc/systemd/system/dokku-daemon.service
	sudo cp -f target/release/dokku-daemon-rs /usr/bin/dokku-daemon
	sudo cp -f conf/dokku-daemon.service /etc/systemd/system/dokku-daemon.service
	sudo mkdir -p /var/run/dokku-daemon
	sudo systemctl daemon-reload
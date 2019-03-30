build: update build-app image

update :
	git pull

build-app : 
	cargo build --release

build-image : 
	sudo docker image rm msc_web || true && \
		sudo docker build -t msc_web .

image : build-image
	sudo docker save msc_web --output msc_web.tar && \
		sudo chown mick:mick msc_web.tar && \
		zip msc_web.tar.zip msc_web.tar && \
		rm msc_web.tar

clean : 
	rm msc_web.tar msc_web.tar.zip||true && cargo clean

SHELL := /bin/bash
l = "127.0.0.1"
p = "6653,6633"
ofp = "13"

install:
	cargo install --path "."

uninstall:
	cargo uninstall tenjin_sdn

run:
	cargo r -- run -p $(p) -l $(l) ctrl$(ofp)

release:
	cargo r -r -- run -p $(p) -l $(l)

build:
	cargo b -r

clean:
	cargo clean

topo = 2
mn:
	sudo mn --controller=remote,ip=127.0.0.1 --mac --switch=ovsk,protocols=OpenFlow$(ofp) --topo=tree,$(topo)
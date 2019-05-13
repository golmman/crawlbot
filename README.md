# crawlbot

## how to use

### run crawl web server

1. clone crawl and checkout a working commit
    1. `git clone https://github.com/crawl/crawl.git`
    1. `cd crawl`
    1. `git checkout 1a43be200669514182634035c87d5750787eaa9c`
1. disable compression
    1. goto `crawl/crawl-ref/source/webserver/ws_handler.py`
    1. change line 137 to `self.subprotocol = "no-compression"`
    1. between line 190 and 191 add `self.deflate = False`
1. [read the webserver preparations](https://github.com/crawl/crawl/blob/master/crawl-ref/source/webserver/README)
1. [build crawl](https://github.com/crawl/crawl/blob/master/crawl-ref/INSTALL.txt)
1. switch to tornado 4.5.3: `pip install 'tornado==4.5.3' --force-reinstall`
1. run the webserver: `( cd crawl-ref/source/ && python webserver/server.py )`

### run crawlbot

1. clone it
1. run `./run.sh`
1. type `/start`, hit return
1. type `/u`, hit return
1. open `http://localhost:8080/` with your browser
1. spectate crawlbot
1. 

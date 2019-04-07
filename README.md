# crawlbot

## how to use

### run crawl web server

* https://github.com/crawl/crawl/blob/master/crawl-ref/INSTALL.txt
* https://github.com/crawl/crawl/blob/master/crawl-ref/source/webserver/README
* Use tornado 4.5.3
  * pip install 'tornado==4.5.3' --force-reinstall

```
make WEBTILES=y
cd crawl-ref/source/ && python webserver/server.py
```

./crawl -webtiles-socket "zzz.sock" -await-connection

Socket url: ws://localhost:8080

https://github.com/websockets/ws


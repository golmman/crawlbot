# crawlbot

## how to use

### run crawl web server

* https://github.com/crawl/crawl/blob/master/crawl-ref/INSTALL.txt
* https://github.com/crawl/crawl/blob/master/crawl-ref/source/webserver/README
* Use tornado 4.5.3
  * pip install 'tornado==4.5.3' --force-reinstall

## graphviz doc
https://www.graphviz.org/doc/info/shapes.html


```
make WEBTILES=y
cd crawl-ref/source/ && python webserver/server.py

INPUT
{"msg":"msgs","messages":[{"text":"<lightred>A kobold comes into view.<lightgrey>","turn":1,"channel":33},{"text":"<lightgrey>The kobold shouts!<lightgrey>","turn":1,"channel":9}]}

OUTPUT
{"msg":"login","username":"crawlbot","password":"123"}
{"msg":"play","game_id":"dcss-web-trunk"}
{"msg":"key","keycode":-253} // down
{"msg":"key","keycode":11} // return
{"msg":"key","keycode":9} // auto-attack
{"msg":"key","keycode":16} // show history
{"msg":"key","keycode":17} // abandon
{"msg":"key","keycode":19} // save and exit
{"msg":"key","keycode":24} // surroundings
{"msg":"key","keycode":27} // escape
{"msg": "input","text": "o"} // explore

// PICK MINOTAUR FIGHTER AXE
{"msg": "input","text": "b"}
{"msg": "input","text": "a"}
{"msg": "input","text": "c"}

// PICK TROLL BERSERKER CLAWS
{"msg": "input","text": "g"}
{"msg": "input","text": "h"}
{"msg": "input","text": "f"}

// ABANDON
{"msg":"key","keycode":17}
{"msg":"input","text":"yes\r"}
{"msg":"input","text":"\r"}
{"msg":"input","text":"\r"}
{"msg":"input","text":"\r"}

{"msg": "ping"}
{"msg": "pong"}
```

Keycodes  
* left -252
* right -251
* up -254
* down -253
* return 11
* esc 27
* ctrl-a 1 ?? (b=2, c=3, ...)
* ctrl-o 15
* ctrl-p 16
* ctrl-q 17
* ctrl-s 19
* tab 9


./crawl -webtiles-socket "zzz.sock" -await-connection

Socket url: ws://localhost:8080

https://github.com/websockets/ws  
https://github.com/websockets-rs/rust-websocket/blob/master/examples/client.rs


Async stdin:
https://ticki.github.io/blog/making-terminal-applications-in-rust-with-termion/


/crawl-ref/settings/old_unicode_glyphs.txt
/crawl-ref/source/defines.h

* DCHAR_WALL: #
* DCHAR_PERMAWALL: ▓
* DCHAR_WALL_MAGIC: *
* DCHAR_FLOOR: .
* DCHAR_FLOOR_MAGIC: ,
* DCHAR_DOOR_OPEN: '
* DCHAR_DOOR_CLOSED: +
* DCHAR_TRAP: ^
* DCHAR_STAIRS_DOWN: >
* DCHAR_STAIRS_UP: <
* DCHAR_GRATE: #
* DCHAR_ALTAR: _
* DCHAR_ARCH: ∩
* DCHAR_FOUNTAIN: ⌠
* DCHAR_WAVY: ≈
* DCHAR_STATUE: 8
* DCHAR_INVIS_EXPOSED: {
* DCHAR_ITEM_DETECTED: ∆
* DCHAR_ITEM_ORB: 0
* DCHAR_ITEM_RUNE: φ
* DCHAR_ITEM_WEAPON: )
* DCHAR_ITEM_ARMOUR: [
* DCHAR_ITEM_WAND: /
* DCHAR_ITEM_FOOD: %
* DCHAR_ITEM_SCROLL: ?
* DCHAR_ITEM_RING: =
* DCHAR_ITEM_POTION: !
* DCHAR_ITEM_MISSILE: (
* DCHAR_ITEM_BOOK: :
* DCHAR_ITEM_STAFF: |
* DCHAR_ITEM_ROD: \
* DCHAR_ITEM_MISCELLANY: }
* DCHAR_ITEM_CORPSE: †
* DCHAR_ITEM_SKELETON: ÷
* DCHAR_ITEM_GOLD: $
* DCHAR_ITEM_AMULET: "
* DCHAR_CLOUD: §
* DCHAR_CLOUD_WEAK: ☼
* DCHAR_CLOUD_FADING: ○
* DCHAR_CLOUD_TERMINAL: °
* DCHAR_TREE: ♣
* DCHAR_TELEPORTER: ©
* DCHAR_TRANSPORTER: ©
* DCHAR_TRANSPORTER_LANDING: ©
* DCHAR_SPACE:  
* DCHAR_FIRED_BOLT: #
* DCHAR_FIRED_ZAP: *
* DCHAR_FIRED_BURST: ÷
* DCHAR_FIRED_DEBUG: X
* DCHAR_FIRED_MISSILE: `
* DCHAR_EXPLOSION: #
* DCHAR_FRAME_HORIZ: ═
* DCHAR_FRAME_VERT: ║
* DCHAR_FRAME_TL: ╔
* DCHAR_FRAME_TR: ╗
* DCHAR_FRAME_BL: ╚
* DCHAR_FRAME_BR: ╝
* DCHAR_DRAW_HORIZ: ─
* DCHAR_DRAW_VERT: │
* DCHAR_DRAW_SLASH: /
* DCHAR_DRAW_BACKSLASH: \
* DCHAR_DRAW_TL: ┌
* DCHAR_DRAW_TR: ┐
* DCHAR_DRAW_BL: └
* DCHAR_DRAW_BR: ┘
* DCHAR_DRAW_DOWN: V
* DCHAR_DRAW_UP: Λ
* DCHAR_DRAW_RIGHT: >
* DCHAR_DRAW_LEFT: <

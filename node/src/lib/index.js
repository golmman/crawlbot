const WebSocket = require('ws');

function affirmHealth(pingTimeout) {
    if (pingTimeout) {
        pingTimeout.refresh();
    }
}

function sendMessage(client, message) {
    client.send(message);
    console.log(`WEBSOCKET SENT: ${message}`);
}

const client = new WebSocket('ws://localhost:8080/socket');

// The value for the connection timeout is based on connection_timeout in webserver/config.py
const connectionTimeout = (600 + 10) * 1000;
const pingTimeout = setTimeout(() => {
    console.log('WEBSOCKET EXIT: did not receive ping in time');
    client.terminate();
    process.exit(0);
}, connectionTimeout);

client.on('open', () => {
    console.log('WEBSOCKET OPEN: connected to server');
    affirmHealth(pingTimeout);
});

client.on('message', (message) => {
    console.log(`WEBSOCKET RECV: ${message}`);

    const { msgs } = JSON.parse(message);

    msgs.forEach((msg) => {
        if (msg.msg === 'ping') {
            affirmHealth(pingTimeout);
            sendMessage(client, JSON.stringify(
                { msg: 'pong' },
            ));
        } else if (msg.msg === 'close') {
            console.log('WEBSOCKET EXIT: server closed connection');
            client.terminate();
            process.exit(0);
        }
    });
});

process.stdin.setEncoding('utf8');

process.stdin.on('readable', () => {
    let chunk = process.stdin.read();
    while (chunk !== null) {
        sendMessage(client, chunk);
        chunk = process.stdin.read();
    }
});

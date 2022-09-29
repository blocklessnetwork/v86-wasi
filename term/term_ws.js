function TermWS(addr, bus) {
    this.addr = addr;
    this.bus = bus;
    this.is_connected = false;
    var that = this;
    this.bus.register("keyboard-code", function(c) {
        that.send_kb(c);
    });
}

TermWS.prototype.connect = function() {
    this.ws = new WebSocket(this.addr);
    this.ws.binaryType = "arraybuffer";
    var self = this;
    this.ws.onclose = function() {
        console.log("on close connect after 100ms");
        if (self.is_connected) {
            bus.send("screen-clear");
        }
        self.is_connected = false;
        setTimeout(function(){
            self.connect();
        }, 100);
    }

    this.ws.onopen = function() {
        self.is_connected = true;
    }

    this.ws.onmessage = function(event) {
        this.is_connected = true;
        self.onmessage(event);
    }
}

TermWS.prototype.send_kb = function(code) {
    var buf = new Uint8Array(3);
    buf[0] = 0x00;
    buf[1] = 0x01;
    buf[2] = code;
    this.ws.send(buf);
}

function putc_decode(p, buf) {
    var row = buf[p]|buf[p+1]<<8;
    p += 2;
    var col = buf[p]|buf[p+1]<<8;
    p += 2;
    var ch = buf[p];
    p += 1;
    var bg_color = buf[p]|buf[p+1]<<8|buf[p+2]<<16|buf[p+3]<<24;
    p += 4;
    var fg_color = buf[p]|buf[p+1]<<8|buf[p+2]<<16|buf[p+3]<<24;
    p += 4;
    return [13, [row, col, ch, bg_color, fg_color]];
}

function tuple8_decode(p, buf) {
    var v1 = buf[p];
    var v2 = buf[p+1];
    p = 2;
    return [2, [v1, v2]];
}

function tuple16_decode(p, buf) {
    var v1 = buf[p]|buf[p+1]<<8;
    p += 2;
    var v2 = buf[p]|buf[p+1]<<8;
    p += 2;
    return [4, [v1, v2]];
}


TermWS.prototype.onmessage = function(event) {
    if (event.data instanceof ArrayBuffer) {
        var buf = new Uint8Array(event.data);
        var p = 0;
        var ct = 0;
        while (p < buf.byteLength - 1) {
            var msg_id = buf[p]|buf[p+1]<<8;
            p += 2;
            if (msg_id === 1) {
                this.bus.send("screen-set-mode", buf[2]);
                p += 1;
                continue;
            }

            if (msg_id === 2) {
                var rs = putc_decode(p, buf);
                p += rs[0];
                this.bus.send("screen-put-char", rs[1]);
                continue;
            }

            if (msg_id == 3) {
                var rs = tuple16_decode(p, buf);
                p += rs[0];
                this.bus.send("screen-set-size-text", rs[1]);
                continue;
            }

            if (msg_id === 4) {
                var rs = tuple8_decode(p, buf);
                p += rs[0];
                this.bus.send("screen-update-cursor-scanline", rs[1]);
                continue;
            }
            dbg_assert(msg_id);
        }
    } 
}



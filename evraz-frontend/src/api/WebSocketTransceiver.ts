export default class WebSocketTransceiver extends EventTarget {
    private static instance : WebSocketTransceiver;

    private ws : WebSocket;

    private constructor(ws: WebSocket) {
        super();
        this.ws = ws;
        this.ws.onmessage = this.onMessage.bind(this);
    }

    public static async getInstance(wsUrl: string) : Promise<WebSocketTransceiver> {
        if (WebSocketTransceiver.instance) {
            return WebSocketTransceiver.instance;
        }
        let ws = new WebSocket(wsUrl);
        ws.onclose = () => {
            console.warn("WS disconnected");
        };
        ws.onerror = (event) => {
            console.error("WS error:",event);
        };

        let sm = new WebSocketTransceiver(ws);
        await sm.waitForOpen();

        WebSocketTransceiver.instance = sm;
        return sm;
    }

    private waitForOpen() {
        return new Promise<void>((resolve, reject) => {
            if (this.ws.readyState === WebSocket.OPEN) {
                resolve();
                return;
            }
           this.ws.onopen = () => {
               resolve();
           }
           setTimeout(reject, 1500);
        });
    }

    private onMessage(event: MessageEvent) {
        const message = JSON.parse(event.data);
        switch (message.method) {
            // WIP
        }
    }
}
import React from "react";
import { WS_URL } from "../api/config";
import WebSocketTransceiver from "../api/WebSocketTransceiver";

const TransceiverContext = React.createContext(await WebSocketTransceiver.getInstance(WS_URL));

export default TransceiverContext;
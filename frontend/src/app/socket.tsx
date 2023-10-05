"use client";

import LedMatrix from "@/components/LedMatrix";
import { MessageType } from "@/types/generated/MessageType";
import { Pixel } from "@/types/generated/Pixel";
import { useEffect, useState } from "react";

interface Socket {
  ip: string;
}

export default function Socket({ ip }: Socket) {
  const [ledData, setLedData] = useState<Pixel[]>([]);
  const [socket, setSocket] = useState(new WebSocket(`ws://${ip}/ws`));
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    if (ledData.length) return;

    const onOpen = () => {
      setIsConnected(true);
    };

    if (!isConnected) {
      socket.addEventListener("open", onOpen);
      return;
    } else {
      socket.removeEventListener("open", onOpen);
    }

    let interval = setInterval(() => {
      sendLedDataRequest(socket);
    }, 100);

    const onMessage = (event: MessageEvent<string>) => {
      let matches = event.data.match(/^\[ERR:(.*)?\]$/);
      if (matches) {
        let errorMessage = matches[1];
        console.error(errorMessage);
        throw new Error(errorMessage);
      }

      const eventData: MessageType = JSON.parse(event.data);

      switch (eventData["type"]) {
        case "Command": {
          let errorMessage = "Received command from lamp which is invalid.";
          console.error(errorMessage);
          throw new Error(errorMessage);
        }
        case "Data": {
          let data = eventData.content;
          switch (data.type) {
            case "LedData": {
              console.log(data.content);
              setLedData(data.content);
              break;
            }
          }
          break;
        }
      }
    };
    socket.addEventListener("message", onMessage);

    return () => {
      socket.removeEventListener("open", onOpen);
      socket.removeEventListener("message", onMessage);
      clearInterval(interval);
    };
  }, [socket, ledData, isConnected]);

  if (!ledData.length) return <h1>Loading...</h1>;

  return <LedMatrix data={ledData} />;
}

function sendLedDataRequest(socket: WebSocket) {
  const command: MessageType = {
    type: "Command",
    content: { type: "GetLedData" },
  };
  socket.send(JSON.stringify(command));
}

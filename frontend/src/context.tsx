"use client";

import { Dispatch, SetStateAction, createContext, useState } from "react";

interface Context {
  socket: WebSocket;
  setSocket: Dispatch<SetStateAction<WebSocket>>;
}

export const context = createContext<Context>({} as Context);

interface ContextProvider {
  children: any | any[];
  ip: string;
}

export function ContextProvider({ children, ip }: ContextProvider) {
  const [socket, setSocket] = useState(new WebSocket(`ws://${ip}/ws`));

  return (
    <context.Provider value={{ socket, setSocket }}>
      {children}
    </context.Provider>
  );
}


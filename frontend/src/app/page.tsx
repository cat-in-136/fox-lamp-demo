import Debug from "@/components/Debug";
import LedData from "@/components/LedData";
import { headers } from "next/headers";
import Socket from "./socket";

export default async function Page() {
  const ip = headers().get("fox-lamp-ip-address") as string;

  // let socket = new WebSocket(`ws://${ip}/test`);

  // socket.onopen = function (e) {};
  // socket.onclose = socket.onerror = function (e) {};
  // socket.onmessage = function (e) {
  //   console.log(e.data);
  // };
  // socket.send("This is a test meow :3")
  return (
    <>
      <Debug />
      {/* <LedData lampIp={ip} /> */}
      <Socket ip={ip} />
    </>
  );
}

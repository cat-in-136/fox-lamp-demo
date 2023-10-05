import { headers as getHeaders } from "next/headers";

export default function Debug() {
  const headers = getHeaders();
  const ip = headers.get("fox-lamp-ip-address") as string;

  return (
    <>
      <p>
        Lamp IP:&nbsp;<span>{ip}</span>
      </p>
      <pre>{JSON.stringify(headers, null, 2)}</pre>
    </>
  );
}

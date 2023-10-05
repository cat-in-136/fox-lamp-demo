import "./globals.css";
import { ReactNode } from "react";

interface Layout {
  children: ReactNode;
}

export default function Layout({ children }: Layout) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}

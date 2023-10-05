if (process.env.NODE_ENV !== "development") throw new Error("Implement prod env");

/** @type {import('next').NextConfig} */
const nextConfig = {
  assetPrefix: process.env.NODE_ENV === "development" ? getDevIp() : "",
};

module.exports = nextConfig;

function getDevIp() {
  const { networkInterfaces } = require("os");

  const nets = networkInterfaces();
  const results = Object.create(null);

  for (const name of Object.keys(nets)) {
    for (const net of nets[name]) {
      // Skip over non-IPv4 and internal (i.e. 127.0.0.1) addresses
      // 'IPv4' is in Node <= 17, from 18 it's a number 4 or 6
      const familyV4Value = typeof net.family === "string" ? "IPv4" : 4;
      if (net.family === familyV4Value && !net.internal) {
        if (!results[name]) {
          results[name] = [];
        }
        results[name].push(net.address);
      }
    }
  }

  const ip = results["en0"][0];

  console.log(`Local dev ip: ${ip}`);

  return `http://${ip}:3000`;
}

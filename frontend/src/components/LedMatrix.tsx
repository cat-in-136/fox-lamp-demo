import Led from "./Led";

interface LedMatrix {
  data: { r: number; g: number; b: number }[];
}

export default function LedMatrix({ data }: LedMatrix) {
  return (
    <div style={{ display: "grid", gridTemplateColumns: "repeat(10, 2rem)", gap: "0.25rem" }}>
      {data.map(({ r, g, b }, i) => {
        return (
          <Led
            key={i}
            color={`#${numToHex(r)}${numToHex(g)}${numToHex(b)}`}
          />
        );
      })}
    </div>
  );
}

function numToHex(number: number) {
  return (number * 10).toString(16).padStart(2, "0");
}

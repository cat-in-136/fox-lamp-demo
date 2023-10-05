interface Led {
  color: string;
}

export default function Led({ color }: Led) {
  return (
    <div
      style={{
        width: "2rem",
        height: "2rem",
        border: "1px solid var(--color)",
        background: color,
        transition: "background 1000ms ease",
      }}
    />
  );
}

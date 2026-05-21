import { useMemo } from "react";
import { Treemap as RechartTreemap, ResponsiveContainer } from "recharts";
import type { ScanResult, ScanCategory } from "../lib/types";
import { formatBytes } from "../lib/format";
import styles from "./Treemap.module.css";

const CATEGORY_COLORS: Record<ScanCategory, string> = {
  temp: "var(--color-temp)",
  cache: "var(--color-cache)",
  logs: "var(--color-logs)",
  duplicates: "var(--color-duplicates)",
  residue: "var(--color-residue)",
};

const CATEGORY_LABELS: Record<ScanCategory, string> = {
  temp: "Temp",
  cache: "Cache",
  logs: "Logs",
  duplicates: "Duplicates",
  residue: "Residue",
};

interface TreemapProps {
  result: ScanResult;
  onCategoryClick: (cat: ScanCategory | null) => void;
}

interface CustomContentProps {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  name?: string;
  value?: number;
  root?: boolean;
  depth?: number;
  colors?: Record<string, string>;
}

function CustomContent({
  x = 0,
  y = 0,
  width = 0,
  height = 0,
  name = "",
  value = 0,
  depth = 0,
}: CustomContentProps) {
  if (depth === 0 || width < 10 || height < 10) return null;
  const cat = name as ScanCategory;
  const color = CATEGORY_COLORS[cat] ?? "#888";
  const showText = width > 60 && height > 30;

  return (
    <g>
      <rect
        x={x + 1}
        y={y + 1}
        width={width - 2}
        height={height - 2}
        fill={color}
        fillOpacity={0.85}
        rx={4}
        ry={4}
      />
      {showText && (
        <>
          <text
            x={x + width / 2}
            y={y + height / 2 - 8}
            textAnchor="middle"
            fill="#fff"
            fontSize={13}
            fontWeight={600}
          >
            {CATEGORY_LABELS[cat] ?? name}
          </text>
          <text
            x={x + width / 2}
            y={y + height / 2 + 10}
            textAnchor="middle"
            fill="rgba(255,255,255,0.8)"
            fontSize={11}
          >
            {formatBytes(value)}
          </text>
        </>
      )}
    </g>
  );
}

export default function Treemap({ result, onCategoryClick }: TreemapProps) {
  const data = useMemo(() => {
    const categories = new Map<ScanCategory, number>();
    for (const entry of result.entries) {
      categories.set(entry.category, (categories.get(entry.category) ?? 0) + entry.size);
    }
    return Array.from(categories.entries())
      .map(([name, size]) => ({ name, size }))
      .filter((d) => d.size > 0)
      .sort((a, b) => b.size - a.size);
  }, [result]);

  if (data.length === 0) return null;

  return (
    <div className={styles.container}>
      <ResponsiveContainer width="100%" height={240}>
        <RechartTreemap
          data={data}
          dataKey="size"
          nameKey="name"
          content={<CustomContent />}
          onClick={(item) => {
            if (item && item.name) {
              onCategoryClick(item.name as ScanCategory);
            }
          }}
        />
      </ResponsiveContainer>
      <div className={styles.legend}>
        {data.map(({ name, size }) => (
          <button
            key={name}
            className={styles.legendItem}
            onClick={() => onCategoryClick(name as ScanCategory)}
          >
            <span
              className={styles.legendDot}
              style={{ background: CATEGORY_COLORS[name as ScanCategory] }}
            />
            <span className={styles.legendName}>{CATEGORY_LABELS[name as ScanCategory] ?? name}</span>
            <span className={styles.legendSize}>{formatBytes(size)}</span>
          </button>
        ))}
      </div>
    </div>
  );
}

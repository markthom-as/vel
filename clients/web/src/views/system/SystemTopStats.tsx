type SystemTopStat = {
  label: string;
  value: string;
};

export function SystemTopStats({
  items,
}: {
  items: SystemTopStat[];
}) {
  return (
    <div className="grid gap-2 rounded-[18px] border border-[var(--vel-color-border)] bg-[rgba(255,255,255,0.02)] px-3 py-3 sm:grid-cols-2 xl:grid-cols-4">
      {items.map((item) => (
        <div key={item.label} className="rounded-[12px] bg-[rgba(255,255,255,0.018)] px-2.5 py-2">
          <p className="text-[9px] uppercase tracking-[0.18em] text-[var(--vel-color-dim)]">
            {item.label}
          </p>
          <p className="mt-1 text-[13px] font-medium leading-5 text-[var(--vel-color-text)]">
            {item.value}
          </p>
        </div>
      ))}
    </div>
  );
}

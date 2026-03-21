interface NavbarStatusProps {
  dateTimeLabel: string;
  contextLine: string;
}

export function NavbarStatus({ dateTimeLabel, contextLine }: NavbarStatusProps) {
  return (
    <div className="min-w-0 pb-0.5">
      <div className="flex min-w-0 items-baseline gap-2 whitespace-nowrap">
        <p className="shrink-0 text-[11px] font-medium tracking-[0.06em] text-zinc-300 [font-variant-caps:small-caps]">
          {dateTimeLabel}
        </p>
        <p className="truncate text-[11px] leading-snug text-zinc-400 sm:text-xs">{contextLine}</p>
      </div>
    </div>
  );
}

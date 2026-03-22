import { IntegrationBrandIcon, type BrandIntegrationKey } from '../../core/Icons';
import { PanelDenseRow } from '../../core/PanelChrome';

export function InfoStat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-xl border border-zinc-800 bg-zinc-950/70 p-3">
      <p className="text-[11px] uppercase tracking-[0.18em] text-zinc-500">{label}</p>
      <p className="mt-2 text-sm text-zinc-100">{value}</p>
    </div>
  );
}

export function IntegrationStatCard({
  title,
  status,
  detail,
  brand,
}: {
  title: string;
  status: string;
  detail: string;
  brand?: BrandIntegrationKey;
}) {
  return (
    <PanelDenseRow>
      <div className="flex items-start justify-between gap-3">
        <div className="flex min-w-0 items-center gap-2">
          {brand ? (
            <span className="inline-flex h-6 w-8 shrink-0 items-center justify-center text-zinc-200">
              <IntegrationBrandIcon brand={brand} normalizeHeight />
            </span>
          ) : null}
          <p className="min-w-0 text-sm font-medium text-zinc-100">{title}</p>
        </div>
        <span className="rounded-full border border-zinc-800 px-2 py-1 text-[10px] uppercase tracking-[0.16em] text-zinc-400">
          {status}
        </span>
      </div>
      <p className="mt-2 text-xs text-zinc-400">{detail}</p>
    </PanelDenseRow>
  );
}

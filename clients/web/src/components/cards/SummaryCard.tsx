import { CardLayout } from './CardLayout';

interface SummaryContent {
  title: string;
  timeframe?: string;
  top_items?: string[];
  recommended_actions?: string[];
}

export function SummaryCardView({ content }: { content: SummaryContent }) {
  return (
    <CardLayout kind="summary_card">
      <div className="font-medium text-zinc-200">{content.title}</div>
      {content.timeframe && <div className="text-xs text-zinc-500 mt-1">{content.timeframe}</div>}
      {content.top_items && content.top_items.length > 0 && (
        <ul className="text-sm text-zinc-400 mt-2 space-y-0.5">
          {content.top_items.map((item, i) => (
            <li key={i}>• {item}</li>
          ))}
        </ul>
      )}
      {content.recommended_actions && content.recommended_actions.length > 0 && (
        <div className="mt-2 pt-2 border-t border-zinc-700">
          <div className="text-xs text-zinc-500 mb-1">Recommended</div>
          <ul className="text-sm text-emerald-400/90 space-y-0.5">
            {content.recommended_actions.map((a, i) => (
              <li key={i}>→ {a}</li>
            ))}
          </ul>
        </div>
      )}
    </CardLayout>
  );
}

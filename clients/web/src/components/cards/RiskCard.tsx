import { CardLayout } from './CardLayout';

interface RiskContent {
  commitment_title: string;
  risk_level: string;
  risk_score?: number;
  top_drivers?: string[];
  dependency_ids?: string[];
  proposed_next_step?: string;
}

export function RiskCardView({ content }: { content: RiskContent }) {
  const levelClass =
    content.risk_level === 'high' || content.risk_level === 'danger'
      ? 'text-amber-400'
      : content.risk_level === 'medium'
        ? 'text-yellow-500'
        : 'text-zinc-400';
  return (
    <CardLayout kind="risk_card">
      <div className="font-medium text-zinc-200">{content.commitment_title}</div>
      <div className={`text-sm mt-1 ${levelClass}`}>Risk: {content.risk_level}</div>
      {typeof content.risk_score === 'number' && (
        <div className="text-xs mt-1 text-zinc-500">
          Score: {content.risk_score.toFixed(2)}
        </div>
      )}
      {content.top_drivers && content.top_drivers.length > 0 && (
        <ul className="text-sm text-zinc-400 mt-2 list-disc list-inside">
          {content.top_drivers.map((d, i) => (
            <li key={i}>{d}</li>
          ))}
        </ul>
      )}
      {content.dependency_ids && content.dependency_ids.length > 0 && (
        <div className="text-xs mt-2 text-zinc-500">
          Dependencies: {content.dependency_ids.join(', ')}
        </div>
      )}
      {content.proposed_next_step && (
        <p className="text-sm text-emerald-400/90 mt-2">{content.proposed_next_step}</p>
      )}
    </CardLayout>
  );
}

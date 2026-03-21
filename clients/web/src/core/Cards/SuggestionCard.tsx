import { CardLayout } from './CardLayout';

interface SuggestionContent {
  suggestion_text: string;
  linked_goal?: string;
  expected_benefit?: string;
}

export function SuggestionCardView({ content }: { content: SuggestionContent }) {
  return (
    <CardLayout kind="suggestion_card">
      <p className="text-zinc-200">{content.suggestion_text}</p>
      {content.linked_goal && (
        <div className="text-xs text-zinc-500 mt-1">Goal: {content.linked_goal}</div>
      )}
      {content.expected_benefit && (
        <div className="text-xs text-emerald-400/80 mt-1">Benefit: {content.expected_benefit}</div>
      )}
    </CardLayout>
  );
}

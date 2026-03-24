import type { ActionItemData } from '../../../types';
import {
  PanelItemInlineActions,
  PanelItemInlineLayout,
  PanelItemMain,
  PanelItemMetaRow,
  PanelItemShell,
  PanelItemSummary,
  PanelItemTitle,
} from '../../../core/PanelItem';
import { ActionChipButton, ProjectTag } from '../../../core/FilterToggleTag';

export function ActionRow({
  item,
  onOpenThread,
}: {
  item: ActionItemData;
  onOpenThread?: (conversationId: string) => void;
}) {
  return (
    <PanelItemShell surface="risk" as="div">
      <PanelItemInlineLayout>
        <PanelItemMain>
          <PanelItemMetaRow>
            <PanelItemTitle as="p" size="sm">
              {item.title}
            </PanelItemTitle>
            {item.project_label ? (
              <ProjectTag label={item.project_label}>{item.project_label}</ProjectTag>
            ) : null}
          </PanelItemMetaRow>
          <PanelItemSummary spacing="compact">{item.summary}</PanelItemSummary>
        </PanelItemMain>
        <PanelItemInlineActions>
          {item.thread_route?.thread_id ? (
            <ActionChipButton
              onClick={() => onOpenThread?.(item.thread_route?.thread_id as string)}
              aria-label={item.thread_route.label}
            >
              {item.thread_route.label}
            </ActionChipButton>
          ) : null}
        </PanelItemInlineActions>
      </PanelItemInlineLayout>
    </PanelItemShell>
  );
}

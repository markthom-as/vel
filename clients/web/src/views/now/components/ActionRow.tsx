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
import { FilterDenseTag, FilterPillButton } from '../../../core/FilterToggleTag';
import { projectTagClasses } from '../nowModel';

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
              <FilterDenseTag className={projectTagClasses(item.project_label)}>{item.project_label}</FilterDenseTag>
            ) : null}
          </PanelItemMetaRow>
          <PanelItemSummary spacing="compact">{item.summary}</PanelItemSummary>
        </PanelItemMain>
        <PanelItemInlineActions>
          {item.thread_route?.thread_id ? (
            <FilterPillButton
              onClick={() => onOpenThread?.(item.thread_route?.thread_id as string)}
              aria-label={item.thread_route.label}
            >
              {item.thread_route.label}
            </FilterPillButton>
          ) : null}
        </PanelItemInlineActions>
      </PanelItemInlineLayout>
    </PanelItemShell>
  );
}

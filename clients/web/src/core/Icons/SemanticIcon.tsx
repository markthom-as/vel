import type { IconProps } from './IconGlyphs';
import {
  CalendarIcon,
  FolderIcon,
  LayersIcon,
  OpenThreadIcon,
  ServerIcon,
  SettingsIcon,
  SparkIcon,
  SyncIcon,
  TagIcon,
  ThreadsIcon,
  WarningIcon,
} from './IconGlyphs';
import type { SemanticIconKey } from '../Theme/semanticRegistry';

export function SemanticIcon({
  icon,
  ...props
}: IconProps & {
  icon: SemanticIconKey;
}) {
  switch (icon) {
    case 'calendar':
      return <CalendarIcon {...props} />;
    case 'folder':
      return <FolderIcon {...props} />;
    case 'layers':
      return <LayersIcon {...props} />;
    case 'open_thread':
      return <OpenThreadIcon {...props} />;
    case 'server':
      return <ServerIcon {...props} />;
    case 'settings':
      return <SettingsIcon {...props} />;
    case 'sync':
      return <SyncIcon {...props} />;
    case 'tag':
      return <TagIcon {...props} />;
    case 'threads':
      return <ThreadsIcon {...props} />;
    case 'warning':
      return <WarningIcon {...props} />;
    case 'spark':
    default:
      return <SparkIcon {...props} />;
  }
}

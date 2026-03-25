import type { SVGProps } from 'react';
import {
  AlertTriangle,
  Calendar,
  CalendarClock,
  CalendarSync,
  ChevronLeft,
  ChevronRight,
  Circle,
  CircleCheck,
  ClockPlus,
  ClipboardCheck,
  Clock,
  Copy,
  Eye,
  FileText,
  Folder,
  Image,
  LayoutGrid,
  Info,
  Inbox,
  Layers,
  MapPin,
  MessagesSquare,
  Mic,
  Minus,
  Monitor,
  Paperclip,
  Plus,
  RefreshCw,
  Search,
  Send,
  Server,
  Settings,
  Smartphone,
  Sparkles,
  SquareArrowOutUpRight,
  Tag,
  Archive,
  User,
  X,
} from 'lucide-react';

export type IconProps = SVGProps<SVGSVGElement> & {
  size?: number;
  strokeWidth?: number;
};

const defaults = { size: 18, strokeWidth: 1.9 };

export function InfoCircleIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Info aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function ChevronLeftIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <ChevronLeft aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function ChevronRightIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <ChevronRight aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function SparkIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Sparkles aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function InboxIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Inbox aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function ThreadsIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <MessagesSquare aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function SettingsIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Settings aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function SyncIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <RefreshCw aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function WarningIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <AlertTriangle aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function CheckCircleIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <CircleCheck aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function ClockIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Clock aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function CalendarIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Calendar aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function TagIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Tag aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function PersonIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <User aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function OpenThreadIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <SquareArrowOutUpRight aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function RescheduleIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <CalendarClock aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function CalendarSyncIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <CalendarSync aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function ClockPlusIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <ClockPlus aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function SendArrowIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Send aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function PlusIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Plus aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function AttachmentIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Paperclip aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function FileIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <FileText aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function ImageIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Image aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function CopyIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Copy aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function EyeIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Eye aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function ArchiveIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Archive aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function CloseIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <X aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function MinimizeIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Minus aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function MicIcon(props: IconProps & { listening?: boolean }) {
  const { listening, size = defaults.size, strokeWidth = defaults.strokeWidth, ...rest } = props;
  void listening;
  return <Mic aria-hidden size={size} strokeWidth={strokeWidth} {...rest} />;
}

export function SearchIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Search aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function ClipboardCheckIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <ClipboardCheck aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function LayersIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Layers aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function LayoutGridIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <LayoutGrid aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function FolderIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Folder aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function MonitorIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Monitor aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function SmartphoneIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Smartphone aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function ServerIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <Server aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function MapPinIcon({ size = defaults.size, strokeWidth = defaults.strokeWidth, ...props }: IconProps) {
  return <MapPin aria-hidden size={size} strokeWidth={strokeWidth} {...props} />;
}

export function DotIcon({ size = defaults.size, className, ...props }: IconProps) {
  return (
    <Circle
      aria-hidden
      size={size}
      className={className}
      fill="currentColor"
      stroke="none"
      strokeWidth={0}
      {...props}
    />
  );
}

export * from './chat';
export * from './context';
export * from './operator';

import { chatQueryKeys } from './chat';
import { contextQueryKeys } from './context';
import { operatorQueryKeys } from './operator';

export const queryKeys = {
  ...chatQueryKeys,
  ...contextQueryKeys,
  ...operatorQueryKeys,
} as const;

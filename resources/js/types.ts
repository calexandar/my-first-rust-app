import type { PageProps } from '@inertiajs/core';

export type SharedProps = PageProps & {
  appName?: string;
  environment?: string;
};

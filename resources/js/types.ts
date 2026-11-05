import type { PageProps } from '@inertiajs/core';

export type AuthUser = {
  id: number;
  name: string;
  is_admin: boolean;
};

export type SharedProps = PageProps & {
  appName?: string;
  environment?: string;
  csrfToken?: string;
  auth?: {
    user: AuthUser | null;
  };
};

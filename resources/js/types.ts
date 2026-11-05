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

export type AdminUser = {
  id: number;
  name: string;
  email: string;
  is_admin: boolean;
  created_at: string;
};

export type UserFilters = {
  q: string;
  role: 'all' | 'admin' | 'member';
};

export type Pagination = {
  current_page: number;
  last_page: number;
  per_page: number;
  total: number;
  from: number | null;
  to: number | null;
};

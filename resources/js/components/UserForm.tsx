import { Link, useForm, usePage } from '@inertiajs/react';
import type { FormEvent } from 'react';

import type { AdminUser, SharedProps } from '../types';

type Props = {
  action: string;
  method?: 'post' | 'put' | 'patch';
  user?: AdminUser;
  isSelf?: boolean;
  submitLabel: string;
};

type UserFormData = {
  name: string;
  email: string;
  password: string;
  password_confirmation: string;
  is_admin: boolean;
};

const fieldClass =
  'w-full rounded-lg border border-stone-300 bg-white px-3 py-2 text-sm text-stone-900 shadow-sm outline-none transition focus:border-blue-500 focus:ring-2 focus:ring-blue-200 disabled:cursor-not-allowed disabled:bg-stone-100';

export default function UserForm({
  action,
  method = 'post',
  user,
  isSelf = false,
  submitLabel,
}: Props) {
  const { props } = usePage<SharedProps>();
  const csrfToken = props.csrfToken ?? '';
  const form = useForm<UserFormData>({
    name: user?.name ?? '',
    email: user?.email ?? '',
    password: '',
    password_confirmation: '',
    is_admin: user?.is_admin ?? false,
  });

  const submit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const options = {
      headers: { 'X-CSRF-TOKEN': csrfToken },
      onError: () => form.reset('password', 'password_confirmation'),
    };

    if (method === 'put') {
      form.put(action, options);
    } else if (method === 'patch') {
      form.patch(action, options);
    } else {
      form.post(action, options);
    }
  };

  return (
    <form
      onSubmit={submit}
      noValidate
      className="space-y-5 rounded-2xl border border-stone-200 bg-white p-6 shadow-sm"
    >
      <div className="space-y-2">
        <label htmlFor="name" className="block text-sm font-medium">
          Name
        </label>
        <input
          id="name"
          name="name"
          type="text"
          autoComplete="name"
          value={form.data.name}
          onChange={(event) => form.setData('name', event.target.value)}
          disabled={form.processing}
          aria-invalid={Boolean(form.errors.name) || undefined}
          aria-describedby={form.errors.name ? 'name-error' : undefined}
          className={fieldClass}
        />
        {form.errors.name && (
          <p id="name-error" role="alert" className="text-sm text-red-600">
            {form.errors.name}
          </p>
        )}
      </div>

      <div className="space-y-2">
        <label htmlFor="email" className="block text-sm font-medium">
          Email
        </label>
        <input
          id="email"
          name="email"
          type="email"
          autoComplete="email"
          value={form.data.email}
          onChange={(event) => form.setData('email', event.target.value)}
          disabled={form.processing}
          aria-invalid={Boolean(form.errors.email) || undefined}
          aria-describedby={form.errors.email ? 'email-error' : undefined}
          className={fieldClass}
        />
        {form.errors.email && (
          <p id="email-error" role="alert" className="text-sm text-red-600">
            {form.errors.email}
          </p>
        )}
      </div>

      <div className="grid gap-5 sm:grid-cols-2">
        <div className="space-y-2">
          <label htmlFor="password" className="block text-sm font-medium">
            Password
          </label>
          <input
            id="password"
            name="password"
            type="password"
            autoComplete="new-password"
            value={form.data.password}
            onChange={(event) => form.setData('password', event.target.value)}
            disabled={form.processing}
            aria-invalid={Boolean(form.errors.password) || undefined}
            aria-describedby={
              form.errors.password ? 'password-error' : 'password-hint'
            }
            className={fieldClass}
          />
          <p id="password-hint" className="text-xs text-stone-500">
            {user
              ? 'Leave blank to keep the current password.'
              : 'Use at least 12 characters.'}
          </p>
          {form.errors.password && (
            <p
              id="password-error"
              role="alert"
              className="text-sm text-red-600"
            >
              {form.errors.password}
            </p>
          )}
        </div>

        <div className="space-y-2">
          <label
            htmlFor="password_confirmation"
            className="block text-sm font-medium"
          >
            Confirm password
          </label>
          <input
            id="password_confirmation"
            name="password_confirmation"
            type="password"
            autoComplete="new-password"
            value={form.data.password_confirmation}
            onChange={(event) =>
              form.setData('password_confirmation', event.target.value)
            }
            disabled={form.processing}
            aria-invalid={
              Boolean(form.errors.password_confirmation) || undefined
            }
            aria-describedby={
              form.errors.password_confirmation
                ? 'password-confirmation-error'
                : undefined
            }
            className={fieldClass}
          />
          {form.errors.password_confirmation && (
            <p
              id="password-confirmation-error"
              role="alert"
              className="text-sm text-red-600"
            >
              {form.errors.password_confirmation}
            </p>
          )}
        </div>
      </div>

      <div className="space-y-2">
        <label className="flex items-start gap-3 rounded-lg border border-stone-200 px-3 py-3">
          <input
            id="is_admin"
            name="is_admin"
            type="checkbox"
            checked={form.data.is_admin}
            onChange={(event) =>
              form.setData('is_admin', event.target.checked)
            }
            disabled={form.processing || isSelf}
            aria-describedby="is-admin-hint"
            className="mt-0.5 size-4 rounded border-stone-300 text-blue-600 focus:ring-blue-300"
          />
          <span>
            <span className="block text-sm font-medium">Administrator</span>
            <span id="is-admin-hint" className="block text-xs text-stone-500">
              {isSelf
                ? 'You cannot remove your own admin access.'
                : 'Administrators can manage every account.'}
            </span>
          </span>
        </label>
        {form.errors.is_admin && (
          <p id="is-admin-error" role="alert" className="text-sm text-red-600">
            {form.errors.is_admin}
          </p>
        )}
      </div>

      <div className="flex flex-wrap items-center gap-3">
        <button
          type="submit"
          disabled={form.processing}
          className="inline-flex items-center justify-center rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-60"
        >
          {form.processing ? 'Saving...' : submitLabel}
        </button>
        <Link
          href="/admin/users"
          className="text-sm font-medium text-stone-600 transition hover:text-stone-900"
        >
          Cancel
        </Link>
      </div>
    </form>
  );
}

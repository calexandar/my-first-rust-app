import { Head, Link, useForm, usePage } from '@inertiajs/react';
import type { FormEvent } from 'react';

import type { SharedProps } from '../../types';

type RegisterForm = {
  name: string;
  email: string;
  password: string;
  password_confirmation: string;
};

const fieldClass =
  'w-full rounded-lg border border-stone-300 bg-white px-3 py-2 text-sm text-stone-900 shadow-sm outline-none transition focus:border-blue-500 focus:ring-2 focus:ring-blue-200 disabled:cursor-not-allowed disabled:bg-stone-100';

export default function Register() {
  const { props } = usePage<SharedProps>();
  const csrfToken = props.csrfToken ?? '';
  const form = useForm<RegisterForm>({
    name: '',
    email: '',
    password: '',
    password_confirmation: '',
  });

  const submit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    form.post('/register', {
      headers: { 'X-CSRF-TOKEN': csrfToken },
      onError: () => form.reset('password', 'password_confirmation'),
    });
  };

  return (
    <>
      <Head title="Register" />
      <section className="mx-auto max-w-md space-y-6">
        <header className="space-y-2">
          <h1 className="text-3xl font-bold tracking-tight">Create an account</h1>
          <p className="text-sm text-stone-500">
            Passwords are hashed with Argon2id and never stored in plain text.
          </p>
        </header>

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
              Use at least 12 characters.
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

          <button
            type="submit"
            disabled={form.processing}
            className="inline-flex w-full items-center justify-center rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-60"
          >
            {form.processing ? 'Creating account...' : 'Create account'}
          </button>
        </form>

        <p className="text-center text-sm text-stone-500">
          Already have an account?{' '}
          <Link href="/login" className="font-medium text-blue-600">
            Log in
          </Link>
        </p>
      </section>
    </>
  );
}

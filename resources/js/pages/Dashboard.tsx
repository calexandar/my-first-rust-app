import { Head, Link, usePage } from '@inertiajs/react';

import type { SharedProps } from '../types';

export default function Dashboard() {
  const { props } = usePage<SharedProps>();
  const user = props.auth?.user ?? null;

  return (
    <>
      <Head title="Dashboard" />
      <section className="space-y-6">
        <header className="space-y-2">
          <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
          <p className="text-lg text-stone-700">
            {user ? `Welcome, ${user.name}.` : 'Welcome back.'}
          </p>
          <p className="text-sm text-stone-500">
            This page is protected by the Rust auth middleware and marked for
            encrypted browser history.
          </p>
        </header>

        <section
          aria-labelledby="account-heading"
          className="rounded-2xl border border-stone-200 bg-white p-6 shadow-sm"
        >
          <h2 id="account-heading" className="text-lg font-semibold">
            Your account
          </h2>
          <dl className="mt-4 grid gap-4 sm:grid-cols-2">
            <div>
              <dt className="text-xs font-medium uppercase tracking-wide text-stone-500">
                Name
              </dt>
              <dd className="mt-1 text-sm text-stone-900">
                {user?.name ?? 'Unknown'}
              </dd>
            </div>
            <div>
              <dt className="text-xs font-medium uppercase tracking-wide text-stone-500">
                Role
              </dt>
              <dd className="mt-1 text-sm text-stone-900">
                {user?.is_admin ? 'Administrator' : 'Member'}
              </dd>
            </div>
          </dl>
        </section>

        <div className="flex flex-wrap gap-3">
          {user?.is_admin && (
            <Link
              href="/admin/users"
              className="inline-flex items-center justify-center rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2"
            >
              View admin users
            </Link>
          )}
          <Link
            href="/contact"
            className="inline-flex items-center justify-center rounded-lg border border-stone-300 px-4 py-2 text-sm font-semibold text-stone-700 transition hover:border-stone-400 hover:bg-stone-50 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2"
          >
            Go to contact
          </Link>
        </div>
      </section>
    </>
  );
}

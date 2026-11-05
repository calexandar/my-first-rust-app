import { Head, Link } from '@inertiajs/react';

import type { AdminUser } from '../../../types';

type Props = {
  user: AdminUser;
};

export default function Show({ user }: Props) {
  return (
    <>
      <Head title={user.name} />
      <section className="mx-auto max-w-2xl space-y-6">
        <header className="flex flex-wrap items-start justify-between gap-4">
          <div className="space-y-2">
            <Link
              href="/admin/users"
              className="text-xs font-medium text-stone-500 transition hover:text-stone-800"
            >
              ← Back to users
            </Link>
            <h1 className="text-3xl font-bold tracking-tight">{user.name}</h1>
            <p className="text-sm text-stone-500">{user.email}</p>
          </div>
          <Link
            href={`/admin/users/${user.id}/edit`}
            className="inline-flex items-center justify-center rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2"
          >
            Edit user
          </Link>
        </header>

        <section
          aria-labelledby="account-heading"
          className="rounded-2xl border border-stone-200 bg-white p-6 shadow-sm"
        >
          <h2 id="account-heading" className="text-lg font-semibold">
            Account
          </h2>
          <dl className="mt-4 grid gap-4 sm:grid-cols-2">
            <div>
              <dt className="text-xs font-medium uppercase tracking-wide text-stone-500">
                Name
              </dt>
              <dd className="mt-1 text-sm text-stone-900">{user.name}</dd>
            </div>
            <div>
              <dt className="text-xs font-medium uppercase tracking-wide text-stone-500">
                Email
              </dt>
              <dd className="mt-1 text-sm text-stone-900">{user.email}</dd>
            </div>
            <div>
              <dt className="text-xs font-medium uppercase tracking-wide text-stone-500">
                Role
              </dt>
              <dd className="mt-1 text-sm text-stone-900">
                {user.is_admin ? 'Administrator' : 'Member'}
              </dd>
            </div>
            <div>
              <dt className="text-xs font-medium uppercase tracking-wide text-stone-500">
                Joined
              </dt>
              <dd className="mt-1 text-sm text-stone-900">
                <time dateTime={user.created_at}>
                  {new Date(user.created_at).toLocaleString()}
                </time>
              </dd>
            </div>
          </dl>
        </section>
      </section>
    </>
  );
}

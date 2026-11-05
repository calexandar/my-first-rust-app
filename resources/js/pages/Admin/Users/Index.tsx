import { Head, Link, router, usePage } from '@inertiajs/react';
import { useState, type FormEvent } from 'react';

import type {
  AdminUser,
  Pagination,
  SharedProps,
  UserFilters,
} from '../../../types';

type Props = {
  users: AdminUser[];
  filters: UserFilters;
  pagination: Pagination;
};

export default function Index({ users, filters, pagination }: Props) {
  const { props } = usePage<SharedProps>();
  const csrfToken = props.csrfToken ?? '';
  const [q, setQ] = useState(filters.q);
  const [role, setRole] = useState<UserFilters['role']>(filters.role);
  const [confirming, setConfirming] = useState<AdminUser | null>(null);
  const [deletingId, setDeletingId] = useState<number | null>(null);

  const visit = (
    page = 1,
    nextQ = q,
    nextRole: UserFilters['role'] = role,
  ) => {
    const params = new URLSearchParams();

    if (nextQ.trim()) params.set('q', nextQ.trim());
    if (nextRole !== 'all') params.set('role', nextRole);
    if (page > 1) params.set('page', String(page));

    const query = params.toString();

    router.get(
      `/admin/users${query ? `?${query}` : ''}`,
      {},
      { preserveState: true, preserveScroll: true, replace: true },
    );
  };

  const submitFilters = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    visit(1);
  };

  const destroy = (user: AdminUser) => {
    setDeletingId(user.id);

    router.delete(`/admin/users/${user.id}`, {
      headers: { 'X-CSRF-TOKEN': csrfToken },
      preserveScroll: true,
      onFinish: () => {
        setDeletingId(null);
        setConfirming(null);
      },
    });
  };

  const range =
    pagination.from && pagination.to
      ? `${pagination.from}–${pagination.to} of ${pagination.total}`
      : '0 accounts';

  return (
    <>
      <Head title="Users" />
      <section className="space-y-6">
        <header className="flex flex-wrap items-end justify-between gap-4">
          <div className="space-y-2">
            <p className="text-xs font-semibold uppercase tracking-wide text-blue-600">
              Admin
            </p>
            <h1 className="text-3xl font-bold tracking-tight">Users</h1>
            <p className="text-sm text-stone-500">
              Create, inspect, update, and remove accounts. Authorization is
              enforced in Rust on every request.
            </p>
          </div>
          <Link
            href="/admin/users/create"
            className="inline-flex items-center justify-center rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2"
          >
            Create user
          </Link>
        </header>

        <form
          onSubmit={submitFilters}
          className="flex flex-col gap-3 rounded-2xl border border-stone-200 bg-white p-4 shadow-sm sm:flex-row sm:items-end"
        >
          <div className="flex-1 space-y-2">
            <label htmlFor="user-search" className="block text-sm font-medium">
              Search
            </label>
            <input
              id="user-search"
              type="search"
              value={q}
              onChange={(event) => setQ(event.target.value)}
              placeholder="Name or email"
              className="w-full rounded-lg border border-stone-300 bg-white px-3 py-2 text-sm text-stone-900 shadow-sm outline-none transition focus:border-blue-500 focus:ring-2 focus:ring-blue-200"
            />
          </div>
          <div className="space-y-2 sm:w-44">
            <label htmlFor="user-role" className="block text-sm font-medium">
              Role
            </label>
            <select
              id="user-role"
              value={role}
              onChange={(event) =>
                setRole(event.target.value as UserFilters['role'])
              }
              className="w-full rounded-lg border border-stone-300 bg-white px-3 py-2 text-sm text-stone-900 shadow-sm outline-none transition focus:border-blue-500 focus:ring-2 focus:ring-blue-200"
            >
              <option value="all">All roles</option>
              <option value="admin">Administrators</option>
              <option value="member">Members</option>
            </select>
          </div>
          <button
            type="submit"
            className="inline-flex items-center justify-center rounded-lg border border-stone-300 px-4 py-2 text-sm font-semibold text-stone-700 transition hover:border-stone-400 hover:bg-stone-50 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2"
          >
            Filter
          </button>
        </form>

        <div className="overflow-hidden rounded-2xl border border-stone-200 bg-white shadow-sm">
          <div className="flex items-center justify-between border-b border-stone-200 px-5 py-4">
            <h2 className="text-lg font-semibold">Accounts</h2>
            <span className="text-xs text-stone-500">{range}</span>
          </div>

          {users.length === 0 ? (
            <div className="px-5 py-10 text-center">
              <p className="text-sm font-medium text-stone-700">
                No accounts match these filters.
              </p>
              <p className="mt-1 text-sm text-stone-500">
                Clear the search or create a new user.
              </p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-stone-200 text-left text-sm">
                <thead className="bg-stone-50 text-xs uppercase tracking-wide text-stone-500">
                  <tr>
                    <th scope="col" className="px-5 py-3 font-medium">
                      Name
                    </th>
                    <th scope="col" className="px-5 py-3 font-medium">
                      Email
                    </th>
                    <th scope="col" className="px-5 py-3 font-medium">
                      Role
                    </th>
                    <th scope="col" className="px-5 py-3 font-medium">
                      Joined
                    </th>
                    <th scope="col" className="px-5 py-3 text-right font-medium">
                      Actions
                    </th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-stone-100">
                  {users.map((user) => (
                    <tr key={user.id}>
                      <td className="whitespace-nowrap px-5 py-3 font-medium text-stone-900">
                        {user.name}
                      </td>
                      <td className="whitespace-nowrap px-5 py-3 text-stone-600">
                        {user.email}
                      </td>
                      <td className="whitespace-nowrap px-5 py-3">
                        <span
                          className={
                            user.is_admin
                              ? 'rounded-full bg-blue-50 px-2.5 py-1 text-xs font-medium text-blue-700'
                              : 'rounded-full bg-stone-100 px-2.5 py-1 text-xs font-medium text-stone-600'
                          }
                        >
                          {user.is_admin ? 'Admin' : 'Member'}
                        </span>
                      </td>
                      <td className="whitespace-nowrap px-5 py-3 text-stone-500">
                        <time dateTime={user.created_at}>
                          {new Date(user.created_at).toLocaleDateString()}
                        </time>
                      </td>
                      <td className="whitespace-nowrap px-5 py-3">
                        {confirming?.id === user.id ? (
                          <div className="flex items-center justify-end gap-2">
                            <span className="text-xs text-stone-500">
                              Delete {user.name}?
                            </span>
                            <button
                              type="button"
                              onClick={() => destroy(user)}
                              disabled={deletingId === user.id}
                              className="rounded-md bg-red-600 px-2.5 py-1.5 text-xs font-semibold text-white transition hover:bg-red-700 disabled:cursor-not-allowed disabled:opacity-60"
                            >
                              {deletingId === user.id ? 'Deleting...' : 'Delete'}
                            </button>
                            <button
                              type="button"
                              onClick={() => setConfirming(null)}
                              className="rounded-md border border-stone-300 px-2.5 py-1.5 text-xs font-medium text-stone-600 transition hover:bg-stone-50"
                            >
                              Cancel
                            </button>
                          </div>
                        ) : (
                          <div className="flex items-center justify-end gap-3">
                            <Link
                              href={`/admin/users/${user.id}`}
                              className="text-xs font-medium text-stone-600 transition hover:text-stone-900"
                            >
                              View
                            </Link>
                            <Link
                              href={`/admin/users/${user.id}/edit`}
                              className="text-xs font-medium text-blue-600 transition hover:text-blue-800"
                            >
                              Edit
                            </Link>
                            <button
                              type="button"
                              onClick={() => setConfirming(user)}
                              className="text-xs font-medium text-red-600 transition hover:text-red-800"
                            >
                              Delete
                            </button>
                          </div>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}

          <div className="flex flex-wrap items-center justify-between gap-3 border-t border-stone-200 px-5 py-4">
            <span className="text-xs text-stone-500">
              Page {pagination.current_page} of {pagination.last_page}
            </span>
            <div className="flex gap-2">
              <button
                type="button"
                onClick={() => visit(pagination.current_page - 1)}
                disabled={pagination.current_page <= 1}
                className="rounded-lg border border-stone-300 px-3 py-1.5 text-sm font-medium text-stone-700 transition hover:border-stone-400 hover:bg-stone-50 disabled:cursor-not-allowed disabled:opacity-50"
              >
                Previous
              </button>
              <button
                type="button"
                onClick={() => visit(pagination.current_page + 1)}
                disabled={pagination.current_page >= pagination.last_page}
                className="rounded-lg border border-stone-300 px-3 py-1.5 text-sm font-medium text-stone-700 transition hover:border-stone-400 hover:bg-stone-50 disabled:cursor-not-allowed disabled:opacity-50"
              >
                Next
              </button>
            </div>
          </div>
        </div>
      </section>
    </>
  );
}

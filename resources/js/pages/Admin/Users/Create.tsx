import { Head, Link } from '@inertiajs/react';

import UserForm from '../../../components/UserForm';

export default function Create() {
  return (
    <>
      <Head title="Create user" />
      <section className="mx-auto max-w-2xl space-y-6">
        <header className="space-y-2">
          <Link
            href="/admin/users"
            className="text-xs font-medium text-stone-500 transition hover:text-stone-800"
          >
            ← Back to users
          </Link>
          <h1 className="text-3xl font-bold tracking-tight">Create user</h1>
          <p className="text-sm text-stone-500">
            New accounts start with no sessions. Share the password with the
            user through a secure channel.
          </p>
        </header>

        <UserForm
          action="/admin/users"
          method="post"
          submitLabel="Create user"
        />
      </section>
    </>
  );
}

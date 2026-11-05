import { Head, Link } from '@inertiajs/react';

import UserForm from '../../../components/UserForm';
import type { AdminUser } from '../../../types';

type Props = {
  user: AdminUser;
  isSelf: boolean;
};

export default function Edit({ user, isSelf }: Props) {
  return (
    <>
      <Head title={`Edit ${user.name}`} />
      <section className="mx-auto max-w-2xl space-y-6">
        <header className="space-y-2">
          <Link
            href={`/admin/users/${user.id}`}
            className="text-xs font-medium text-stone-500 transition hover:text-stone-800"
          >
            ← Back to {user.name}
          </Link>
          <h1 className="text-3xl font-bold tracking-tight">Edit user</h1>
          <p className="text-sm text-stone-500">
            Update account details or set a new password. Leave the password
            fields blank to keep the existing password.
          </p>
        </header>

        <UserForm
          action={`/admin/users/${user.id}`}
          method="put"
          user={user}
          isSelf={isSelf}
          submitLabel="Save changes"
        />
      </section>
    </>
  );
}

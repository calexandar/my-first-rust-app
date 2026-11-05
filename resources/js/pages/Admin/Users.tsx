import { Head } from '@inertiajs/react';

type AdminUser = {
  id: number;
  name: string;
  email: string;
  is_admin: boolean;
  created_at: string;
};

type Props = {
  users: AdminUser[];
};

export default function Users({ users }: Props) {
  return (
    <>
      <Head title="Admin · Users" />
      <section className="space-y-6">
        <header className="space-y-2">
          <p className="text-xs font-semibold uppercase tracking-wide text-blue-600">
            Admin
          </p>
          <h1 className="text-3xl font-bold tracking-tight">Users</h1>
          <p className="text-sm text-stone-500">
            Only administrators can load this page. The server enforces the
            check in Rust, not just in the navigation.
          </p>
        </header>

        <div className="overflow-hidden rounded-2xl border border-stone-200 bg-white shadow-sm">
          <div className="flex items-center justify-between border-b border-stone-200 px-5 py-4">
            <h2 className="text-lg font-semibold">Accounts</h2>
            <span className="text-xs text-stone-500">
              {users.length} {users.length === 1 ? 'user' : 'users'}
            </span>
          </div>
          {users.length === 0 ? (
            <p className="px-5 py-8 text-sm text-stone-500">No users yet.</p>
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
                          {new Date(user.created_at).toLocaleString()}
                        </time>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </section>
    </>
  );
}

export default function NavBar() {
  return (
    <nav className="fixed left-0 right-0 top-0 z-50 border-b border-slate-800/80 bg-slate-950/90 backdrop-blur-md">
      <div className="mx-auto flex h-14 max-w-6xl items-center justify-between px-4 sm:px-6">
        <a
          href="/"
          className="flex items-center gap-2 font-bold tracking-tight text-white transition hover:text-slate-200"
        >
          BetterUptime
        </a>

        <div className="hidden items-center gap-1 sm:flex">
          <NavLink href="/">Dashboard</NavLink>
          <NavLink href="/websites">Websites</NavLink>
          <NavLink href="/alerts">Alerts</NavLink>
        </div>

        <div className="flex items-center gap-3">
          <a
            href="/signin"
            className="rounded-xl px-4 py-2 text-sm font-medium text-slate-400 transition hover:bg-slate-800 hover:text-white"
          >
            Sign in
          </a>
          <a
            href="/signup"
            className="rounded-xl bg-gradient-to-r from-cyan-500 to-teal-500 px-4 py-2 text-sm font-semibold text-white shadow-lg shadow-cyan-500/20 transition hover:brightness-110"
          >
            Sign up
          </a>
        </div>
      </div>
    </nav>
  );
}

function NavLink({ href, children }) {
  return (
    <a
      href={href}
      className="rounded-xl px-4 py-2 text-sm font-medium text-slate-400 transition hover:bg-slate-800 hover:text-white"
    >
      {children}
    </a>
  );
}

export default function NavBar() {
  return (
    <nav className="sticky top-0 z-50 border-b border-slate-200/60 bg-white/95 shadow-sm shadow-slate-200/50 backdrop-blur-md">
      <div className="mx-auto flex h-16 max-w-6xl items-center justify-between px-4 sm:px-6 lg:px-8">
        {/* Brand */}
        <a
          href="/"
          className="group flex items-center gap-3 transition-opacity hover:opacity-90"
        >
          <span className="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-teal-500 to-cyan-600 shadow-lg shadow-teal-500/25 ring-2 ring-white/50">
            <span className="text-sm font-bold text-white">↑</span>
          </span>
          <span className="text-lg font-semibold tracking-tight text-slate-800">
            BetterUptime
          </span>
        </a>

        {/* Main links */}
        <div className="hidden items-center gap-0.5 sm:flex">
          <NavLink href="/">Dashboard</NavLink>
          <NavLink href="/websites">Websites</NavLink>
          <NavLink href="/alerts">Alerts</NavLink>
        </div>

        {/* Right side */}
        <div className="flex items-center gap-3">
          <a
            href="/signin"
            className="rounded-lg px-4 py-2.5 text-sm font-medium text-slate-600 transition-colors hover:bg-slate-100 hover:text-slate-900"
          >
            Sign in
          </a>
          <a
            href="/signup"
            className="rounded-xl bg-gradient-to-r from-teal-500 to-cyan-500 px-4 py-2.5 text-sm font-semibold text-white shadow-lg shadow-teal-500/25 transition-all hover:shadow-teal-500/40 hover:brightness-105"
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
      className="rounded-lg px-4 py-2.5 text-sm font-medium text-slate-600 transition-colors hover:bg-slate-100 hover:text-slate-900"
    >
      {children}
    </a>
  );
}

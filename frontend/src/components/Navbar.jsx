export default function NavBar() {
  return (
    <nav className="sticky top-0 z-50 border-b border-slate-200/80 bg-white/90 backdrop-blur-sm">
      <div className="mx-auto flex h-14 max-w-6xl items-center justify-between px-4 sm:px-6">
        {/* Brand */}
        <a href="/" className="flex items-center gap-2 font-semibold text-slate-800">
          <span className="flex h-8 w-8 items-center justify-center rounded-lg bg-emerald-500 text-white">
            ↑
          </span>
          BetterUptime
        </a>

        {/* Main links */}
        <div className="hidden items-center gap-1 sm:flex">
          <NavLink href="/">Dashboard</NavLink>
          <NavLink href="/websites">Websites</NavLink>
          <NavLink href="/alerts">Alerts</NavLink>
        </div>

        {/* Right side */}
        <div className="flex items-center gap-2">
          <a
            href="/signin"
            className="rounded-md px-3 py-2 text-sm font-medium text-slate-600 hover:bg-slate-100 hover:text-slate-900"
          >
            Sign in
          </a>
          <a
            href="/signup"
            className="rounded-md bg-emerald-600 px-3 py-2 text-sm font-medium text-white hover:bg-emerald-700"
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
      className="rounded-md px-3 py-2 text-sm font-medium text-slate-600 hover:bg-slate-100 hover:text-slate-900"
    >
      {children}
    </a>
  );
}

export default function NavBar() {
  return (
    <nav className="fixed top-0 left-0 right-0 z-50 border-b border-slate-800 bg-slate-950/90 backdrop-blur-lg">
      <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-6">

        {/* Logo */}
        <a
          href="/"
          className="flex items-center gap-2 text-lg font-bold text-white tracking-tight hover:opacity-80 transition"
        >
          <span className="bg-gradient-to-r from-cyan-400 to-teal-400 bg-clip-text text-transparent">
            BetterUptime
          </span>
        </a>

        {/* Center Links */}
        <div className="hidden items-center gap-2 md:flex">
          <NavLink href="/">Dashboard</NavLink>
          <NavLink href="/websites">Websites</NavLink>
          <NavLink href="/alerts">Alerts</NavLink>
        </div>

        {/* Right Buttons */}
        <div className="flex items-center gap-3">
          <a
            href="/signin"
            className="rounded-lg px-4 py-2 text-sm font-medium text-slate-400 transition hover:bg-slate-800 hover:text-white"
          >
            Sign in
          </a>

          <a
            href="/signup"
            className="rounded-lg bg-gradient-to-r from-cyan-500 to-teal-500 px-4 py-2 text-sm font-semibold text-white shadow-md shadow-cyan-500/20 transition hover:scale-105 hover:brightness-110"
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
      className="relative rounded-lg px-4 py-2 text-sm font-medium text-slate-400 transition hover:text-white"
    >
      <span className="relative z-10">{children}</span>

      {/* Hover background */}
      <span className="absolute inset-0 rounded-lg bg-slate-800 opacity-0 transition hover:opacity-100"></span>
    </a>
  );
}

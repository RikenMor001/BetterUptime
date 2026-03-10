import {useState} from "react";

export default function NavBar() {
  const [isOpen, setIsOpen] = useState(false);
  const toggleMenu = () => {
    setIsOpen(!isOpen);
  }
  return (
    <nav className="fixed left-0 right-0 top-0 z-50 border-b border-stone-700 bg-stone-900 shadow-lg">
      <div className="mx-auto flex h-14 max-w-6xl items-center justify-between px-4 sm:px-6">
        <a
          href="/"
          className="font-bold tracking-tight text-white transition hover:text-slate-200"
          >
          BetterUptime
        </a>

        <div className="hidden items-center gap-1 sm:flex gap-20 text-stone-200">
          <div className="hover:cursor-pointer hover:text-stone-300 hover:rounded-lg hover:bg-stone-800 hover:px-3 hover:py-2">
            Dashboard
          </div>
          <div className="hover:cursor-pointer hover:text-stone-300 hover:rounded-lg hover:bg-stone-800 hover:px-3 hover:py-2">
            Websites
          </div>
          <div className="hover:cursor-pointer hover:text-stone-300 hover:rounded-lg hover:bg-stone-800 hover:px-3 hover:py-2">
            Alerts
          </div>
        </div>

        <div className="flex items-center gap-3">
            Sign in
          <button onClick={toggleMenu}>
            Sign up
          </button>
        </div>
      </div>
    </nav>
  );
}

function NavLink({ href, children }) {
  return (
    <a
      href={href}
      className="rounded-lg px-4 py-2 text-sm font-medium text-slate-400 transition hover:bg-slate-800 hover:text-white"
    >
      {children}
    </a>
  );
}

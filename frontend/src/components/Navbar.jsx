import {useNavigate} from "react-router-dom";

export default function NavBar() {
  const navigate = useNavigate();

  const handleSingup = () => {
    navigate("/signup");  
  }
  return (
    <nav className="fixed left-0 right-0 top-0 z-50 border-b border-stone-700 bg-black shadow-lg">
      <div className="mx-auto flex h-14 max-w-6xl items-center justify-between px-4 sm:px-6">
        <a
          href="/"
          className="font-bold tracking-tight text-white transition hover:text-slate-200 text-xl"
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
          <button className="rounded-lg bg-black text-white px-3 py-2 hover:cursor-pointer hover:bg-white hover:text-black hover:border-2
          hover:border-black hover:cursor-pointer border-1 border-white text-sm font-semibold hover:font-bold">
            Sign in
          </button>
          <button 
          onClick={handleSingup}
          className="rounded-lg text-black bg-white rounded-lg px-3 py-2 hover:cursor-pointer hover:bg-black hover:text-white
          hover:cursor-pointer border-1 border-white hover:border-white text-sm font-semibold hover:font-bold">
            Sign up
          </button>
        </div>
      </div>
    </nav>
  );
}

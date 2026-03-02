export default function NavBar(){
  return <div>
    <div className="bg-neutral-900 justify-between items-center flex px-4 py-2 shadow-xl fixed w-full z-50 border-b border-neutral-800 h-14">
      <div className="text-slate-100 font-bold text-xl hover:text-slate-200 hover:cursor-pointer transition-transform duration-300 hover:-translate-y-0.5">
        BetterUptime
      </div>
      <div className="flex gap-4">
        <button className="border border-slate-200 shadow-sm rounded-2xl px-4 py-1 text-slate-300 hover:bg-slate-200 hover:cursor-pointer transition-colors duration-300 text-sm font-medium text-center
        hover:text-slate-900 hover:border-white hover:bg-slate-700 hover:text-white font-semibold transition-transform duration-300 hover:-translate-y-0.5">
          Sign in
        </button>
        <button className="border border-slate-200 shadow-sm rounded-2xl px-4 py-1 text-slate-300 hover:bg-slate-200 hover:cursor-pointer transition-colors duration-300 text-sm font-medium text-center
        hover:text-slate-900 hover:border-white hover:bg-slate-700 hover:text-white font-semibold transition-transform duration-300 hover:-translate-y-0.5">
          Sign up
        </button>
      </div>
    </div>
  </div>
}

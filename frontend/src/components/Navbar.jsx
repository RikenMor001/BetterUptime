export default function NavBar(){
  return <div>
    <div className="justify-between items-center flex px-4 py-3 bg-white shadow-xl fixed w-full z-50 border-b border-gray-200">
      <div className="text-slate-700 font-bold text-lg
    hover:text-slate-900 hover:cursor-pointer transition-transform duration-300 hover:-translate-y-0.5">
        BetterUptime
      </div>
      <div>
        <button className="border border-slate-300 shadow-sm rounded-md px-4 py-2 text-slate-600 hover:bg-slate-200 hover:cursor-pointer transition-colors duration-300 text-sm font-medium text-center">
          Sign up
        </button>
      </div>
    </div>
  </div>
}

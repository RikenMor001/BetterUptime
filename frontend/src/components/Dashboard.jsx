export default function Dashboard() {
  return (
    <div className="relative min-h-screen overflow-hidden bg-slate-950">
      <div
        className="absolute inset-0 opacity-30"
        style={{
          backgroundImage: `
            linear-gradient(to bottom, rgb(15 23 42 / 0.95), rgb(2 6 23)),
            linear-gradient(rgb(51 65 85 / 0.15) 1px, transparent 1px),
            linear-gradient(90deg, rgb(51 65 85 / 0.15) 1px, transparent 1px)
          `,
          backgroundSize: '100% 100%, 48px 48px, 48px 48px',
        }}
      />
      <div className="absolute left-1/2 top-1/3 h-[400px] w-[600px] -translate-x-1/2 -translate-y-1/2 rounded-full bg-cyan-500/10 blur-[120px]" />

      <div className="relative flex min-h-screen flex-col items-center justify-center px-4 py-20">
        <div className="max-w-3xl text-center">
          <h1 className="text-4xl font-bold tracking-tight text-white sm:text-5xl lg:text-6xl">
            The most reliable
            <br />
            <span className="bg-gradient-to-r from-cyan-400 to-teal-400 bg-clip-text text-transparent">
              uptime monitoring
            </span>
            <br />
            service
          </h1>
          <p className="mt-6 text-lg leading-relaxed text-slate-400 sm:text-xl">
            Get 10 monitors, 10 heartbeats and a status page
            <br className="hidden sm:inline" />
            with 3-minute checks totally free.
          </p>

          <div className="mt-10 flex flex-row flex-nowrap items-center justify-center gap-3 sm:gap-4">
            <input
              type="email"
              placeholder="Your work e-mail address"
              className="h-12 w-72 shrink-0 rounded-xl border border-slate-600 bg-slate-800/80 px-5 py-2.5 text-slate-100 placeholder:text-slate-500 focus:border-cyan-500/60 focus:outline-none focus:ring-2 focus:ring-cyan-500/30 sm:w-96"
            />
            <button className="h-12 shrink-0 rounded-xl bg-gradient-to-r from-cyan-500 to-teal-500 px-6 font-semibold text-white shadow-lg shadow-cyan-500/20 transition hover:shadow-cyan-500/30 hover:brightness-110">
              Get started for free
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

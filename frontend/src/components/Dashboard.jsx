export default function Dashboard(){
    return <div className="bg-neutral-800 h-screen flex items-center justify-center">
        <div className="text-center text-stone-50">
            <div className="text-center text-stone-50 text-5xl font-bold tracking-tght">
                The most reliable <br/>
                uptime monitoring service 
            </div>
            <div className="text-center text-stone-400 text-xl font-medium mt-4">
                <p>Get 10 monitors, 10 heartbeats and a status page <br/>with 3-minute checks totally free.</p>    
            </div>
            <div className="mt-4 flex flex-row flex-nowrap items-center justify-center gap-4">
                <input 
                    type="email"
                    placeholder="Your work e-mail address"
                    className="w-80 shrink-0 border border-stone-700 rounded-md px-6 py-2 text-stone-50 bg-neutral-900 text-center focus:outline-none focus:ring-2 focus:ring-neutral-500 focus:border-neutral-500 placeholder:text-stone-400 sm:w-96"
                />
                <button className="shrink-0 rounded-md bg-neutral-700 px-6 py-2 text-stone-50 hover:bg-neutral-600 hover:cursor-pointer">
                    Get started for free
                </button>
            </div> 
        </div>
    </div>
}

import { auth } from "@/auth";
import Image from "next/image";

export default async function Home() {
  const session = await auth();

  return (
    <main className='flex min-h-screen flex-col items-center justify-between p-3 pt-24 md:p-24'>
      <section className='z-10 max-w-5xl xl:w-2/4 items-center justify-between font-mono text-sm'>
        <h1 className=' text-center font-semibold text-xl pb-8'> FRESH EYES</h1>
        <div className='flex w-full items-center justify-between border-b border-gray-300 bg-gradient-to-b from-zinc-200 py-4 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  rounded-xl lg:border lg:bg-gray-200 p-4 lg:dark:bg-zinc-800/30'>
          <p className='w-full'>Welcome {session?.user.name}!</p>
          <picture className='flex w-full h-full items-end justify-end'>
            <Image src={session?.user.image || "/vercel.svg"} alt='Vercel Logo' className='rounded-full' width={48} height={48} priority />
          </picture>
        </div>

        <div className='mt-8 flex flex-col gap-5 border-b border-gray-300 bg-gradient-to-b from-zinc-200 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto rounded-xl lg:border lg:bg-gray-200 p-4 md:p-6 lg:dark:bg-zinc-800/30'>
          <h1 className='text-[15px]'>Recreate a pull request with the following details</h1>
          <section className='flex gap-4'>
            <input
              className={`border-[1.5px] bg-secondary-gray border-input-border text-base p-4 rounded-md w-full placeholder:font-medium text-black `}
              placeholder='Repo name'
            />
            <input
              className={`border-[1.5px] bg-secondary-gray border-input-border text-base p-4 rounded-md w-full placeholder:font-medium text-black `}
              placeholder='Owner name'
            />
          </section>
          <input
            className={`border-[1.5px] bg-secondary-gray border-input-border text-base p-4 rounded-md w-full placeholder:font-medium text-black `}
            placeholder='Pull request number'
          />
          <button
            className={`bg- border border-white hover:opacity-70 rounded-md w-full px-12 py-[16px] whitespace-nowrap font-semibold `}
          >
            Create PR
          </button>
        </div>
      </section>
    </main>
  );
}

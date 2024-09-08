import "@/styles/globals.css";
import "@/styles/animated-background.scss"
import Actions from "@/components/ui/actions";
import Navbar from "@/components/main/navbar";
import Player from "@/components/main/player";
import Head from "next/head";
import { PlayerProvider } from "@/context/playerContext";
import { useRouter } from "next/router";
import { Toaster } from "@/components/ui/sonner";
import { ThemeProvider } from "@/components/themeProvider";
import { ScrollArea } from "@/components/ui/scroll-area";
import AnimatedBackground from "@/components/ui/animated-background";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import { InvokeArgs } from "@tauri-apps/api/tauri";


const handler = {
  async send(channel: string, value: unknown) {
    try {
      await emit(channel, value);
    } catch (error) {}
  },
  on(channel: string, callback: (...args: unknown[]) => void) {
    try {
      const unsubscribe = listen(channel, (event) => {
        callback(event.payload);
      });

      return () => {
        unsubscribe.then((unsub) => unsub());
      };
    } catch (error) {}
  },
  // async invoke<T>(channel: string, args?: InvokeArgs) {
  //   try {
  //     if(!navigator.appVersion) return;
  //     const result = await invoke(channel, args);
  //     return result as T;
  //   } catch (error) {
  //     console.error(`Error invoking channel ${channel}:`, error);
  //   }
  // },
};
if(typeof window != "undefined"){
  window.Ipc = handler;
}
export type IpcHandler = typeof handler;

export default function App({ Component, pageProps }) {
  const router = useRouter();

  return (
    <ThemeProvider
      attribute="class"
      defaultTheme="system"
      disableTransitionOnChange
      enableSystem
    >
      <main className="select-none overflow-hidden rounded-lg bg-white font-sans text-xs text-black antialiased dark:bg-black dark:text-white">
        <Head>
          <title>Wora</title>
        </Head>
        {["/setup"].includes(router.pathname) ? (
          <Component {...pageProps} />
        ) : (
          <PlayerProvider>
            <AnimatedBackground/>
            <div className="h-dvh w-dvw">
              <div>
                <Actions />
                <Toaster position="top-right" />
                <div className="flex gap-8">
                  <div className="sticky top-0 z-50 h-dvh p-8 pr-0 pt-12">
                    <Navbar />
                  </div>
                  <div className="h-screen flex-grow p-8 pl-0 pt-10">
                    <div className="wora-transition relative flex h-full w-full flex-col">
                      <ScrollArea className="mt-2.5 h-full w-full gradient-mask-b-80">
                        <Component {...pageProps} />
                        <div className="h-[10vh] w-full"></div>
                      </ScrollArea>
                      <Player />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </PlayerProvider>
        )}
      </main>
    </ThemeProvider>
  );
}

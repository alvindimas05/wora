import { IpcHandler } from "@/pages/_app";
import { InvokeArgs } from "@tauri-apps/api/tauri";

declare global {
  interface Window {
    Ipc: IpcHandler;
    __TAURI__: {
      invoke<T>(cmd: string, args?: InvokeArgs): Promise<T>;
    }
  }
}

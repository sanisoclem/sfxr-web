import { useSignal } from "@preact/signals";
import Generator from "../islands/Generator.tsx";

export default function Home() {
  const count = useSignal(3);
  return (
    <div class="text-white bg-gray-900 min-h-screen flex flex-col justify-center items-center">
      <div class="w-full max-w-screen-lg p-4 flex justify-center flex-col items-center">
        <img
          class=""
          src="/logo.svg"
          width="150"
          height="80"
        />
        <h1 class="mb-8 text-2xl text-center w-full">Sound Effect Generator</h1>
        <div class="flex flex-col gap-4">
          <Generator />
        </div>
      </div>
    </div>
  );
}

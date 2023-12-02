import { type Signal, useSignal } from "@preact/signals";
import { Button } from "../components/Button.tsx";
// @deno-types="../sfxr-web/pkg/sfxr_web.d.ts"
import init, { SoundEffectGenerator } from "../sfxr-web/pkg/sfxr_web.js";
import { useEffect } from "preact/hooks";
import { useRef } from "preact/hooks";

export default function Generator() {
  const generator = useRef<SoundEffectGenerator>();
  const preset = useSignal<string>("");
  const seed = useSignal<bigint>(BigInt("0"));
  const shareSeed = useSignal<string>("");
  const data = useSignal<Array<{ x: number; y: number }>>([]);
  const nav = useEffect(() => {
    const doit = async () => {
      await init((await fetch("/sfxr_web_bg.wasm")).arrayBuffer());
      generator.current = new SoundEffectGenerator();
    };
    setTimeout(() => {
      const split = window.location.pathname.split("/").pop()?.split("-");
      if (split && split.length === 2) {
        loadSeed(split[0].toLowerCase(), BigInt(`0x${split[1]}`));
      }
    }, 200);
    void doit();
  }, []);

  const randomizePreset = (name: string, s?: bigint) => () => {
    const seed = s || randomBigInt();
    window.location.pathname = `/${name.toUpperCase()}-${
      seed.toString(16).toUpperCase()
    }`;
  };

  const loadSeed = (name: string, s: bigint) => {
    seed.value = s;
    preset.value = name;
    shareSeed.value = `${name.toUpperCase()}-${s.toString(16).toUpperCase()}`;

    if (!generator) return;
    generator.current?.preset(preset.value, seed.value);
    dump();
  };

  const mutate = () => {
    if (!preset) return;
    randomizePreset(preset.value);
  };

  const play = () => {
    if (!generator) return;
    generator.current?.play();
  };

  const randomBigInt = (): bigint => {
    const hex = [...Array(8).keys()]
      .map((_) => Math.floor(Math.random() * 255))
      .map((i) => i.toString(16).padStart(2, "0"))
      .join("");

    return BigInt(`0x${hex}`);
  };

  function dump() {
    data.value =
      (generator.current?.export_raw() as number[] | undefined)?.map((v, i) => {
        return {
          x: i,
          y: v,
        };
      }) || [];
  }

  return (
    <>
      <section class="controls w-full flex flex-row justify-center gap-4 flex-wrap">
        <Button onClick={mutate} disabled={!preset}>Mutate</Button>
        <Button onClick={play}>Play</Button>
        <Button>Export</Button>
      </section>
      <section class="seed">
        <div class="text-center" hidden={!shareSeed}>
          <label>
            <input
              class="text-gray-500 tracking-wide p-2 border-solid border-2 border-emerald-500 text-center"
              type="text"
              placeholder="Seed"
              value={shareSeed}
            />
          </label>
        </div>
      </section>
      <section class="wave-display">
      </section>
      <section class="presets w-full flex flex-row justify-center gap-4 flex-wrap">
        <Button onClick={randomizePreset("pickup")}>Pickup</Button>
        <Button onClick={randomizePreset("laser")}>Laser</Button>
        <Button onClick={randomizePreset("explosion")}>Explosion</Button>
        <Button onClick={randomizePreset("powerup")}>Powerup</Button>
        <Button onClick={randomizePreset("hit")}>Hit</Button>
        <Button onClick={randomizePreset("jump")}>Jump</Button>
        <Button onClick={randomizePreset("blip")}>Blip</Button>
      </section>
    </>
  );
}

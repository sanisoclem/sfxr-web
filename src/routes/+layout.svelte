<script lang="ts">
	import '../app.css';
	import logo from '../assets/logo.svg';
  import SoundEffectGenerator from '../components/SoundEffectGenerator.svelte';
</script>

<div class="text-white bg-gray-900 min-h-screen flex flex-col justify-center items-center">
  <div class="w-full max-w-screen-lg p-4 flex justify-center flex-col items-center flex-grow">
    <img alt="square wave" class="" src={logo} width="150" height="80" />
    <h1 class="mb-8 text-2xl text-center w-full">Sound Effect Generator</h1>

    <div class="flex flex-col gap-4">
      <SoundEffectGenerator />
		<slot/>
    </div>
  </div>
	<div
	class="self-end justify-self-end w-full px-8 py-4 flex flex-col md:flex-row gap-x-20 text-center md:text-left gap-y-8 text-xs justify-between"
>
	<div class="flex justify-center flex-col">
		<p>
			This site was made with <a class="external-link" href="https://kit.svelte.dev/">SvelteKit</a>
			and <a class="external-link" href="https://tailwindcss.com/docs">Tailwind CSS</a>, and is
			hosted by <a class="external-link" href="https://pages.cloudflare.com/">Cloudflare Pages</a> 💖
		</p>
	</div>
	<div class="">
		<ul class="flex justify-center gap-x-4 md:gap-x-8">
			<li>
				<a
					href="https://github.com/sanisoclem/sfxr-web"
					class="hover:text-amber-500 flex gap-x-2"
				>
					<svg class="h-8 w-8 fill-current" viewBox="3 3 18 18">
						<g id="code" opacity="0.75">
							<path
								id="left-bracket"
								d="M4,12v-1h1c1,0,1,0,1-1V7.614C6,7.1,6.024,6.718,6.073,6.472C6.127,6.22,6.212,6.009,6.33,5.839
		C6.534,5.56,6.803,5.364,7.138,5.255C7.473,5.14,8.01,5,8.973,5H10v1H9.248c-0.457,0-0.77,0.191-0.936,0.408
		C8.145,6.623,8,6.853,8,7.476v1.857c0,0.729-0.041,1.18-0.244,1.493c-0.2,0.307-0.562,0.529-1.09,0.667
		c0.535,0.155,0.9,0.385,1.096,0.688C7.961,12.484,8,12.938,8,13.665v1.862c0,0.619,0.145,0.848,0.312,1.062
		c0.166,0.22,0.479,0.407,0.936,0.407L10,17l0,0v1H8.973c-0.963,0-1.5-0.133-1.835-0.248c-0.335-0.109-0.604-0.307-0.808-0.591
		c-0.118-0.165-0.203-0.374-0.257-0.625C6.024,16.283,6,15.9,6,15.387V13c0-1,0-1-1-1H4z"
							/>
							<use
								transform="matrix(-1,0,0,1,24,0)"
								id="right-bracket"
								x="0"
								y="0"
								width="24"
								height="24"
								xlink:href="#left-bracket"
							/>
						</g>
					</svg>
					<span class="leading-8">Source</span>
				</a>
			</li>
		</ul>
	</div>
</div>
</div>


<style lang="postcss">
	.fill-transparent {
		fill: none;
	}
	a.external-link {
		@apply hover:text-amber-500 bg-white bg-opacity-10 px-1;
	}
</style>

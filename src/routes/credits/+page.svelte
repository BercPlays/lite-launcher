<script>
	import AttributionCard from '$lib/components/AttributionCard.svelte';
</script>

<div class="flex flex-col relative m-auto p-6 clear-left h-auto text-base-content">
	<div class="text-center select-none">
		<p class="material-symbols-rounded text-rose-600 heartbeat-text">favorite</p>
		<p>Special Thanks</p>
	</div>

	<div class="my-4"></div>
	<div>
		<div class="z-[1] card w-full p-2 shadow bg-primary text-primary-content select-none">
			<div class="card-body justify-center items-center">
				<div>
					<p class="text-lg"><strong>Lite Launcher</strong></p>
					<p class="text-xs">Made by zsigsza</p>
				</div>

				<slot />
			</div>
		</div>
		<div class="divider"></div>
		<span class="grid gap-8 justify-center">
			<AttributionCard title="daisyUI" description="Tailwind component library"></AttributionCard>
			<AttributionCard
				title="Tauri"
				description="Desktop app framework using HTML, CSS and JavaScript with a Rust backend"
			></AttributionCard>
			<AttributionCard title="Tailwind CSS" description="CSS Utility framework"></AttributionCard>
			<AttributionCard title="Svelte" description="Frontend JavaScript framework"></AttributionCard>
			<AttributionCard title="SvelteKit" description="App framework / meta framework using Svelte"
			></AttributionCard>
			<AttributionCard title="Vite" description="Frontend build tool"></AttributionCard>
			<AttributionCard title="CSS Theme Change" description=""></AttributionCard>
			<AttributionCard title="Google Icons" description=""></AttributionCard>
		</span>
	</div>
</div>

<style>
	.grid {
		grid-template-columns: repeat(auto-fit, minmax(15em, 0.16fr));
	}
	@keyframes heartbeat {
		0% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.3);
		}
		100% {
			transform: scale(1);
		}
	}

	.heartbeat-text {
		animation: heartbeat 1s infinite;
	}
</style>

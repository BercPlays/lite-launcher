<script>
	import { onMount } from 'svelte';
	import MenuBar from './MenuBar.svelte';

	/** @type {HTMLButtonElement} */
	export let button;
	export let title = '';

	/** @type {HTMLDialogElement} */
	let modal;

	onMount(async () => {
		//FIXME | Hacky fix
		while (!button) await new Promise((resolve) => setTimeout(resolve, 250));
		button.addEventListener('click', () => {
			modal.showModal();
		});
	});
</script>

<dialog id="basemodal" class="modal" bind:this={modal}>
	<div class="modal-box">
		<h3 class="font-bold text-lg">{title}</h3>
		<div class="divider m-0"></div>
		<slot />
		<div class="modal-action">
			<div class="divider m-0"></div>
			<form method="dialog">
				<slot name="actions" />
			</form>
		</div>
	</div>
</dialog>

<script>
	import BaseDialog from '$lib/components/BaseDialog.svelte';
	import AccountDataItem from '$lib/components/DataComponents/AccountDataItem.svelte';
	import Checkbox from '$lib/components/pageComponents/versions/Checkbox.svelte';

	/** @type {HTMLDialogElement} */
	let modal;

	/** @type {HTMLButtonElement} */
	let add_button;
</script>

<BaseDialog button={add_button} title="Add an account!">
	<button class="btn btn-ghost w-full"
		>Login with <img src="/icons/logo_ms.png" class="w-5 h-5" alt="" />Microsoft</button
	>
	<div class="divider m-0 mb-2">OR</div>
	<label class="input input-bordered flex items-center gap-2">
		Username |
		<input type="text" class="grow" />
	</label>
	<button class="btn btn-primary w-full mt-2">Create account</button>
	<svelte:fragment slot="actions">
		<button class="btn">Close</button>
	</svelte:fragment>
</BaseDialog>

<div class="flex flex-row sticky top-0 bg-base-200 p-4 justify-between">
	<div class="flex flex-row items-center">
		<input
			type="text"
			placeholder="Search accounts..."
			class="input input-bordered w-24 md:w-auto"
		/>
		<div class="divider divider-horizontal m-0 p-0"></div>
		<div class="mx-2 flex flex-col">
			<p>Account type</p>

			<div class="flex flex-row">
				<!-- CHECKBOXES -->
				<Checkbox text="Premium"></Checkbox>
				<Checkbox text="Cracked"></Checkbox>
			</div>
		</div>
	</div>
	<div class="flex items-center">
		<div class="tooltip tooltip-left" data-tip="Add account">
			<button class="btn btn-ghost px-3" bind:this={add_button}
				><span class="material-symbols-rounded">add</span></button
			>
		</div>
	</div>
</div>
<div class="p-2 test overflow-auto">
	<AccountDataItem></AccountDataItem>
</div>

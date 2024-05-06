<script>
	/**
	 * @type {String | undefined}
	 */
	export let text = '';
	/**
	 * @type {boolean | undefined}
	 */
	export let checked = false;
	/**
	 * @type {(arg0 : boolean) => void}
	 */
	export let changed = () => {};

	/**
	 * @type {HTMLInputElement}
	 */
	let checkbox;
</script>

<div class="bg-base-300 mr-4 px-2">
	<label class="cursor-pointer label">
		<input
			type="checkbox"
			bind:checked
			class="checkbox"
			bind:this={checkbox}
			on:change={() => {
				if (changed) changed(checkbox.checked);
			}}
		/>
		<span class="label-text pl-2">{text}</span>
	</label>
</div>

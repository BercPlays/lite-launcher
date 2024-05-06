<script>
	import VersionListItem from '$lib/components/DataComponents/VersionListItem.svelte';
	import VersionTypeCheckbox from '$lib/components/pageComponents/versions/Checkbox.svelte';
	import VirtualList from '@sveltejs/svelte-virtual-list';
	import { invoke } from '@tauri-apps/api/tauri';

	import { onMount } from 'svelte';

	let request = {};
	/**
	 * @type {{id: String, type: String, url: String, time: String, releaseTime: String}[]}
	 */
	let versionList = [];

	onMount(() => {
		request = invoke('get', {
			invokeMessage: 'https://launchermeta.mojang.com/mc/game/version_manifest.json'
		}).then((value) => {
			request = JSON.parse(value);
			// @ts-ignore
			versionList = request['versions'];
		});
	});
</script>

{#await request}
	<div class="relative m-auto clear-left h-auto text-center p-12">
		<span class="loading loading-bars loading-lg"></span>
	</div>
{:then}
	<div class="flex flex-row sticky top-0 bg-base-200 p-4 items-center">
		<input
			type="text"
			placeholder="Search version..."
			class="input input-bordered w-24 md:w-auto"
		/>
		<div class="divider divider-horizontal m-0 p-0"></div>
		<div class="mx-2 flex flex-col">
			<p>Versions</p>

			<div class="flex flex-row">
				<VersionTypeCheckbox text="Releases"></VersionTypeCheckbox>
				<VersionTypeCheckbox text="Snapshot"></VersionTypeCheckbox>
				<VersionTypeCheckbox text="Old Alpha"></VersionTypeCheckbox>
				<VersionTypeCheckbox text="Old Beta"></VersionTypeCheckbox>
			</div>
		</div>
	</div>
	<div class="p-2 test overflow-auto">
		<VirtualList items={versionList} let:item>
			<VersionListItem version={item.id} type={item.type}></VersionListItem>
		</VirtualList>
	</div>
{:catch}
	<div role="alert" class="alert alert-error">
		<span class="material-symbols-rounded">error</span>
		<span>Unable to get versions. Check your connection!</span>
	</div>
{/await}

<style>
	.test {
		height: calc(100svh - 10em);
	}
</style>

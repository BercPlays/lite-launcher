<script>
	import MenuBarListItem from '$lib/components/DataComponents/MenuBarListItem.svelte';
</script>

<div class="drawer drawer-open">
	<input id="side-drawer" type="checkbox" class="drawer-toggle" />
	<div class="drawer-content">
		<slot />
	</div>
	<div class="drawer-side">
		<label for="side-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
		<ol class="menu menu-md p-4 min-h-full bg-base-200 text-base-content">
			<a class="btn btn-ghost text-xl" href="/"><strong class="">Lite Launcher</strong></a>
			<MenuBarListItem name="Launcher Home" path="/" icon="home"></MenuBarListItem>
			<MenuBarListItem name="Instances" path="/instances" icon="deployed_code"></MenuBarListItem>
			<MenuBarListItem name="Accounts" path="/accounts" icon="people"></MenuBarListItem>
			<MenuBarListItem name="Settings" path="/settings" icon="settings"></MenuBarListItem>
			<MenuBarListItem name="Attributions" path="/credits" icon="handshake"></MenuBarListItem>
		</ol>
	</div>
</div>

<style>
	.drawer-side {
		view-transition-name: drawer-side;
	}
</style>

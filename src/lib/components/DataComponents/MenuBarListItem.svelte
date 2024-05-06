<script>
	import { page } from '$app/stores';
	export let path = '/';
	export let name = '';
	export let icon = '';
</script>

<li>
	<a href={path} class={$page.url.pathname == path ? 'active' : ''}>
		<p class="material-symbols-rounded text-xs">{icon}</p>
		<p>{name}</p>
	</a>
</li>

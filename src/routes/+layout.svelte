<script>
	import { onMount } from 'svelte';
	import '../app.css';
	import { themeChange } from 'theme-change';
	import NavigationBar from '$lib/components/NavigationBar.svelte';
	import MenuBar from '$lib/components/MenuBar.svelte';
	import ViewTransition from '$lib/components/effects/ViewTransition.svelte';

	onMount(() => {
		themeChange(false);
	});

	// Prevent F5 and CTRL F5
	// document.addEventListener('keydown', function (event) {
	// 	if (
	// 		event.key === 'F5' ||
	// 		(event.ctrlKey && event.key === 'r') ||
	// 		(event.metaKey && event.key === 'r')
	// 	) {
	// 		event.preventDefault();
	// 	}
	// });

	// Prevent context menu (right click menu)
	// document.addEventListener('contextmenu', function (event) {
	// 	event.preventDefault();
	// });
</script>

<ViewTransition />

<MenuBar>
	<NavigationBar></NavigationBar>
	<slot />
</MenuBar>

<script>
	import Center from './NavbarSections/center.svelte';
	import End from './NavbarSections/end.svelte';
	import Start from './NavbarSections/start.svelte';
</script>

<div class="text-base-content sticky top-0 z-30">
	<nav class="navbar bg-base-200">
		<div class="navbar-start"><Start></Start></div>
		<div class="navbar-center"><Center></Center></div>
		<div class="navbar-end"><End></End></div>
	</nav>
</div>

<style>
	.navbar {
		view-transition-name: navbar;
	}
</style>

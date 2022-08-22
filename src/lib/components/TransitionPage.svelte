<script lang="ts">
	import { fly, blur, slide, fade } from 'svelte/transition';
    import { cubicOut } from "svelte/easing";

	export let url: URL;
</script>

{#key url}
<div class="flex-auto overflow-y-auto"
in:fly={{ x:-100, duration: 300, easing:cubicOut }}
>
<slot />
</div>
{/key}

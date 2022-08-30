<script lang="ts">
	import { fly,fade } from 'svelte/transition';
    import { cubicOut } from "svelte/easing";

	export let url: URL;
</script>

{#key url}
<div class="flex-auto bg-gray-200 py-8 p-16 flex flex-col"
in:fade={{  duration: 500, easing:cubicOut }}
>
<slot />
</div>
{/key}

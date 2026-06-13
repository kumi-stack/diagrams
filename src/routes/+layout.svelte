<script lang="ts">
	import './layout.css';
	import { Toaster } from "$lib/components/ui/sonner";

	const { children } = $props();
</script>

{@render children()}
<Toaster position="bottom-right" richColors />

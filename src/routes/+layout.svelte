<script lang="ts">
	import './layout.css';
	import { listen } from "@tauri-apps/api/event";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	import { Toaster } from "$lib/components/ui/sonner";
	import {
		QUICK_ADD_CREATED_EVENT,
		type QuickAddResult,
	} from "@/api/shell";
	import { openQuickAddResult } from "@/features/projects/quick-add-navigation";

	const { children } = $props();

	onMount(() => {
		if (getCurrentWindow().label !== "main") return;

		const unlisten = listen<QuickAddResult>(
			QUICK_ADD_CREATED_EVENT,
			(event) => void openQuickAddResult(event.payload),
		);
		return () => void unlisten.then((stop) => stop());
	});
</script>

{@render children()}
<Toaster position="bottom-right" richColors />

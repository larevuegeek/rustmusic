<script lang="ts">
import { resolveCoverSrc, type CoverSize } from "$lib/helper/tools/coverHelper";
import FadeImg from "./FadeImg.svelte";

let { path, alt = "", size = 'full', ...rest }: {
    path: string | null | undefined;
    alt?: string;
    size?: CoverSize;
    class?: string;
    [key: string]: any;
} = $props();

let src = $state<string | null>(null);

$effect(() => {
    const currentPath = path;
    if (!currentPath) {
        src = null;
        return;
    }
    resolveCoverSrc(currentPath, size).then(url => {
        src = url;
    });
});
</script>

{#if src}
    <FadeImg {src} {alt} {...rest} />
{/if}

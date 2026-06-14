<script lang="ts">
  	type HeadingLevel = "h1" | "h2" | "h3" | "h4" | "h5" | "h6";

    type Props = {
      title: string;
      as?: HeadingLevel
    };

	let { title, as = "h1" }: Props = $props();

  const sizes: Record<HeadingLevel, string> = {
    h1: "text-3xl",
    h2: "text-2xl",
    h3: "text-xl",
    h4: "text-lg",
    h5: "text-base",
    h6: "text-sm"
  };
</script>

<div>
  	<svelte:element
		this={as} 
    class={`font-bold tracking-tight
            text-neutral-900 dark:text-neutral-100
            ${sizes[as]}`}>
    {title}
    </svelte:element>
  <div class="mt-2 h-1 w-12 rounded-full bg-emerald-600 dark:bg-emerald-400"></div>
</div>
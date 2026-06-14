<script lang="ts">
    import Icon from "@iconify/svelte";


    type Props = {
        extraClass?: string,
        text?: string,
        iconBefore?: string | null,
        iconBeforeSize?: number,
        iconAfter?: string | null,
        iconAfterSize?: number,
        disabled?: boolean,
        active?: boolean,
        onClick?: () => void,
        onDbclick?: () => void,
    };

    let {
        extraClass = "",
        text = "",
        disabled = false,
        active = false,
        iconBefore = null,
        iconBeforeSize = 16,
        iconAfter = null,
        iconAfterSize = 16,
        onClick,
        onDbclick
    }: Props = $props();

    const baseClass =
        "w-full flex gap-3 items-center rounded px-3 py-2 text-left cursor-pointer \
        transition-colors duration-150 \
        hover:bg-white/10 hover:text-white \
        dark:hover:bg-white/10";

    function itemClass(active: boolean, disabled: boolean, extraClass: string) {
        return [
        baseClass,
        active && "bg-white/10",
        disabled && "opacity-50 pointer-events-none",
        extraClass
        ].filter(Boolean).join(" ");
    }
</script>
<button 
    class={itemClass(active, disabled, extraClass)}
    onclick={onClick}
    ondblclick={onDbclick}
    disabled={disabled}
>
    {#if iconBefore !== null}
        <Icon icon={iconBefore} width={iconBeforeSize} height={iconBeforeSize} />
    {/if}
    {text}
    {#if iconAfter !== null}
        <Icon icon={iconAfter} width={iconAfterSize} height={iconAfterSize} />
    {/if}
</button>
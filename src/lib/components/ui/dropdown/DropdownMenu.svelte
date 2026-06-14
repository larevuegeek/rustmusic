<script lang="ts">
    import Icon from "@iconify/svelte";

    let menuEl: HTMLElement | null = null;
    let dropUp = $state(false);
    let menuStyle = $state('');

    type Props = {
        children?: any,
        text?: string,
        icon?: string | null,
        iconSize?: number,
        label?: string,
        isOpen?: boolean
    }

    let { 
        children = null,
        text = "", 
        label = "More",
        icon = null,
        iconSize = 24,
        isOpen = $bindable(false)
    }: Props = $props();

    function toogleOpen() {
        if (!isOpen) calculatePosition();
        isOpen = !isOpen;
    }

    function calculatePosition() {
        if (!menuEl) return;
        const rect = menuEl.getBoundingClientRect();
        const spaceBelow = window.innerHeight - rect.bottom;
        const menuHeight = 200;
        dropUp = spaceBelow < menuHeight;

        if (dropUp) {
            menuStyle = `position:fixed; bottom:${window.innerHeight - rect.top}px; right:${window.innerWidth - rect.right}px;`;
        } else {
            menuStyle = `position:fixed; top:${rect.bottom}px; right:${window.innerWidth - rect.right}px;`;
        }
    }

    const openClass = $derived(
        isOpen
        ? "opacity-100 scale-100 pointer-events-auto"
        : "opacity-0 scale-95 pointer-events-none"
    );

    $effect(() => {
        if (!isOpen) return;

        function handleClickOutside(e: MouseEvent) {
            if (!menuEl) return;
            // Vérifie aussi que le clic n'est pas dans le menu portal
            const target = e.target as Node;
            const portalMenu = document.querySelector('[data-dropdown-menu]');
            if (menuEl.contains(target) || portalMenu?.contains(target)) return;
            isOpen = false;
        }

        // Délai pour laisser le onClick du DropdownItem s'exécuter d'abord
        const timer = setTimeout(() => {
            document.addEventListener('click', handleClickOutside);
        }, 100);

        return () => {
            clearTimeout(timer);
            document.removeEventListener('click', handleClickOutside);
        };
    });
</script>

<div class="relative group/menu" bind:this={menuEl}>
    <button
        onclick={() => toogleOpen()}
        class="p-2 rounded-md cursor-pointer
                text-neutral-500 dark:text-neutral-400
                hover:bg-black/5 dark:hover:bg-white/10"
        aria-label={label}
    >
        {text}
        {#if icon !== null}
            <Icon icon={icon} width={iconSize} height={iconSize} />
        {/if}
    </button>
</div>

{#if isOpen}
    <div
        data-dropdown-menu
        class="{openClass} z-9999 w-48 rounded-md 
               border border-white/10 bg-neutral-950/95 p-1 
               text-sm text-neutral-200 shadow-xl backdrop-blur
               transition"
        style={menuStyle}
    >
        {@render children?.()}
    </div>
{/if}
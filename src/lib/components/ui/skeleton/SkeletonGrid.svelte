<!--
  ═══════════════════════════════════════════════════════════
  SkeletonGrid — Placeholder animé pendant le chargement
  ═══════════════════════════════════════════════════════════

  POURQUOI ?
  Quand une page charge, afficher un écran vide ou un spinner
  donne l'impression que l'app est lente. Un skeleton (fantôme)
  montre la structure de la page avant que les données arrivent.
  L'utilisateur voit que "ça charge" mais comprend déjà la mise
  en page → la perception de vitesse est meilleure.

  USAGE :
  <SkeletonGrid count={10} type="album" />
  <SkeletonGrid count={8} type="artist" />

  L'animation "pulse" fait varier l'opacité en boucle,
  ce qui indique visuellement que le contenu est en cours
  de chargement (pattern standard Material/Apple).
-->

<script lang="ts">
  /**
   * count : combien de cartes fantômes afficher
   *         En général, on met le même nombre que ce qui tient
   *         sur un écran (5-10) pour remplir visuellement l'espace
   *
   * type : "album" (carré) ou "artist" (rond)
   *        Change la forme du placeholder image
   */
  let { count = 8, type = 'album' }: { count?: number; type?: 'album' | 'artist' } = $props();
</script>

<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 2xl:grid-cols-6 min-[1800px]:grid-cols-7 min-[2200px]:grid-cols-8 gap-6">
  <!--
    {#each Array(count)} crée un tableau de `count` éléments undefined
    On ne se sert pas de la valeur (elle est `undefined`), juste de
    l'index `i` pour la clé du each.

    Le `(i)` est la clé Svelte — elle permet à Svelte de savoir
    quel élément est quel. Ici c'est juste l'index car on ne
    réordonne jamais les skeletons.
  -->
  {#each Array(count) as _, i (i)}
    <div class="animate-pulse flex flex-col gap-3">
      <!--
        animate-pulse : animation CSS Tailwind qui fait
        osciller l'opacité entre 100% et 50%

        Le conteneur flex-col + gap-3 reproduit la structure
        exacte de AlbumListItem / ArtistListItem
      -->

      <!-- Placeholder image -->
      <div
        class="aspect-square bg-neutral-200 dark:bg-neutral-800
               {type === 'artist' ? 'rounded-full' : 'rounded-xl'}"
      ></div>

      <!-- Placeholder titre (barre grise 75% de largeur) -->
      <div class="h-4 bg-neutral-200 dark:bg-neutral-800 rounded w-3/4"></div>

      <!-- Placeholder sous-titre (barre grise 50% de largeur) -->
      <div class="h-3 bg-neutral-200 dark:bg-neutral-800 rounded w-1/2"></div>
    </div>
  {/each}
</div>

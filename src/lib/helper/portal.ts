/**
 * Action Svelte pour "porter" (teleporter) un élément vers `document.body`.
 *
 * Utile pour les modales / popins qui doivent échapper aux stacking contexts
 * et containing blocks créés par des ancêtres avec `transform`, `filter`,
 * `overflow: hidden`, etc. — un `position: fixed` piégé dans un tel ancêtre
 * est positionné par rapport à celui-ci au lieu du viewport.
 *
 * Usage :
 *   <div use:portal>
 *     <div class="fixed inset-0 ...">Modal content</div>
 *   </div>
 */

export function portal(node: HTMLElement, target: HTMLElement | string = document.body) {
  const targetEl =
    typeof target === "string" ? document.querySelector<HTMLElement>(target) : target;
  const dest = targetEl ?? document.body;
  dest.appendChild(node);

  return {
    destroy() {
      if (node.parentNode) {
        node.parentNode.removeChild(node);
      }
    },
  };
}

export function h(tag, props, children) {
  return { tag, props, children };
}

export function unmount() {}
export function mount(vnode, container) {
  const el = document.createElement(vnode.tag);

  el.textContent = vnode.children;
  el.className = vnode.props.class;

  container.appendChild(el);

  console.log("mount", vnode, container);
}
export function patch() {}
